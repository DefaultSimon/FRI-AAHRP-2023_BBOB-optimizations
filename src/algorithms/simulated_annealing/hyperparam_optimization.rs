use std::cmp::max;
use std::slice::Iter;
use clap::error::ContextValue::String;

use itertools::{iproduct, min};
use miette::miette;
use num::{abs, Num, ToPrimitive};

use crate::algorithms::simulated_annealing::neighborhood_generation::SANeighborhood;
use crate::algorithms::simulated_annealing::options::SAOptions;
use crate::algorithms::simulated_annealing::simulated_annealing::run_sa;
use crate::core::problem::BBOBProblem;

use self::Option::*;

pub fn get_optimal_params(problem: &mut BBOBProblem, current_options: SAOptions) -> SAOptions {
    let mut min_options = current_options.clone();
    let mut min_value = run_sa(problem, min_options.clone()).unwrap().value;
    let mut value_changes = get_value_changes(min_value, current_options, problem);
    let mut new_options = current_options.clone();

    for ch in value_changes.clone().iter() {
        println!("Option: {}, Diff: {}, Direction: {}", ch.option.name(), ch.option_diff, ch.direction.name());
    }

    for i in 0..1 {
        match value_changes[i].option {
            InitialTemperature => {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { initial_temperature: current_options.initial_temperature + 2, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { initial_temperature: current_options.initial_temperature - 2, ..new_options } }
                }
            }
            AnnealingSchedule =>  {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { annealing_schedule: current_options.annealing_schedule + 0.01, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { annealing_schedule: current_options.annealing_schedule - 0.02, ..new_options } }
                }
            }
            MaxIterationsSa =>  {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { max_iterations_sa: current_options.max_iterations_sa + 500, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { max_iterations_sa: current_options.max_iterations_sa - 500, ..new_options } }
                }
            }
            MaxIterationsLs =>  {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { max_iterations_ls: current_options.max_iterations_ls + 1000, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { max_iterations_ls: current_options.max_iterations_ls - 1000, ..new_options } }
                }
            }
            InitialStepSizeSa =>  {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { initial_step_size_sa: current_options.initial_step_size_sa + 0.1, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { initial_step_size_sa: current_options.initial_step_size_sa - 0.1, ..new_options } }
                }
            }
            InitialStepSizeLs =>  {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { initial_step_size_ls: current_options.initial_step_size_ls + 0.01, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { initial_step_size_ls: current_options.initial_step_size_ls * 0.5, ..new_options } }
                }
            }
            MinTemp =>  {
                match value_changes[i].direction {
                    Direction::Positive => { new_options = SAOptions { min_temp: current_options.min_temp + 2.0, ..new_options } }
                    Direction::Negative => { new_options = SAOptions { min_temp: current_options.min_temp * 0.1, ..new_options } }
                }
            }
            BestNSa => {
                match value_changes[i].direction {
                    Direction::Positive => { if current_options.n_best_sa < 39 {new_options = SAOptions { n_best_sa: current_options.n_best_sa + 1, ..new_options }}}
                    Direction::Negative => { new_options = SAOptions { n_best_sa: current_options.n_best_sa - 1, ..new_options } }
                }
            }
            BestNLs => {
                match value_changes[i].direction {
                    Direction::Positive => { if current_options.n_best_ls < 39 {new_options = SAOptions { n_best_ls: current_options.n_best_ls + 1, ..new_options }}}
                    Direction::Negative => { new_options = SAOptions { n_best_ls: current_options.n_best_ls - 1, ..new_options } }
                }
            }
        }
    }

    SAOptions { function: problem.name, ..new_options }
}

fn get_value_changes(base_value: f64, options: SAOptions, problem: &mut BBOBProblem) -> Vec<OptionValue> {
    let mut option_value_changes = Vec::new();
    for option in Option::iterator() {
        match option {
            //InitialTemperature => { option_value_changes.push(get_min_temp_change(base_value, options.min_temp, problem)) }
            //AnnealingSchedule => { option_value_changes.push(get_anneačing_schedule_change(base_value, options.annealing_schedule, problem)) }
            //MaxIterationsSa => { option_value_changes.push(get_max_iter_sa_change(base_value, options.max_iterations_sa, problem)) }
            //MaxIterationsLs => { option_value_changes.push(get_max_iter_ls_change(base_value, options.max_iterations_ls, problem)) }
            //InitialStepSizeSa => { option_value_changes.push(get_initial_step_change_sa(base_value, options.initial_step_size_sa, problem)) }
            InitialStepSizeLs => { option_value_changes.push(get_initial_step_change_ls(base_value, options.initial_step_size_ls, problem)) }
            //MinTemp => { option_value_changes.push(get_min_temp_change(base_value, options.min_temp, problem)) }
            //BestNSa => { option_value_changes.push(get_best_n_sa_change(base_value, options.n_best_sa, problem)) }
            //BestNLs => { option_value_changes.push(get_best_n_ls_change(base_value, options.n_best_ls, problem)) }
            _ => continue
        }
    }

    option_value_changes.sort_by(|el1, el2| el2.option_diff.total_cmp(&el1.option_diff));
    option_value_changes
}

fn generate_neighborhood(base_value: f64, options: SAOptions, problem: &mut BBOBProblem) -> Vec<SAOptions> {
    let value_changes = get_value_changes(base_value, options, problem);
    let mut res = Vec::new();
    for i in 0..2 {
        res.append(&mut generate_neighbors(options.clone(), value_changes[i].option.clone()))
    }

    res
}

fn generate_neighbors(options: SAOptions, to_change: Option) -> Vec<SAOptions> {
    let mut new_options = Vec::new();

    match to_change {
        InitialTemperature => {
            for i in -5..5.to_i32().unwrap() {
                new_options.push(SAOptions { initial_temperature: (options.initial_temperature as i32 + i * 2) as u8, ..SAOptions::default() })
            }
        }
        AnnealingSchedule => {
            for i in -4..15.to_i32().unwrap() {
                new_options.push(SAOptions { annealing_schedule: options.annealing_schedule + i as f64 * 0.01, ..SAOptions::default() })
            }
        }
        MaxIterationsSa => {
            for i in -10..10.to_i32().unwrap() {
                new_options.push(SAOptions { max_iterations_sa: (options.max_iterations_sa as i32 + i * 1000) as u64, ..SAOptions::default() })
            }
        }
        MaxIterationsLs => {
            for i in -10..10.to_i32().unwrap() {
                new_options.push(SAOptions { max_iterations_ls: (options.max_iterations_ls as i32 + i * 1000) as u64, ..SAOptions::default() })
            }
        }
        InitialStepSizeSa => {
            for i in -10..10.to_i32().unwrap() {
                new_options.push(SAOptions { initial_step_size_sa: options.initial_step_size_sa + i as f64 * 0.1, ..SAOptions::default() })
            }
        }
        InitialStepSizeLs => {
            for i in -10..10.to_i32().unwrap() {
                new_options.push(SAOptions { initial_step_size_ls: options.initial_step_size_ls + i as f64 * 0.1, ..SAOptions::default() })
            }
        }
        MinTemp => {
            for i in -5..5.to_i32().unwrap() {
                new_options.push(SAOptions { min_temp: options.min_temp + i as f64, ..SAOptions::default() })
            }
        }
        BestNSa => {
            for i in 0..8 {
                new_options.push(SAOptions { initial_temperature: options.initial_temperature + i, ..SAOptions::default() })
            }
        }
        BestNLs => {
            for i in 0..8 {
                new_options.push(SAOptions { initial_temperature: options.initial_temperature + i, ..SAOptions::default() })
            }
        }
    }

    new_options
}

fn get_initial_step_change_ls(base_value: f64, initial_step: f64, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { initial_step_size_ls: initial_step * 0.1, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { initial_step_size_ls: initial_step + 0.1, ..SAOptions::default() });

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if problem.name.global_minimum() - pos_val > problem.name.global_minimum() - neg_val {
        direction = Direction::Positive
    }

    println!("Base: {}, Positive: {}, negative: {}, pos_diff: {}, neg_diff: {}", problem.name.global_minimum(), pos_val, neg_val, problem.name.global_minimum() - pos_val, problem.name.global_minimum() - neg_val);
    let option_diff = f64::max((problem.name.global_minimum() - pos_val).abs(), (problem.name.global_minimum() - neg_val).abs());
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_initial_step_change_sa(base_value: f64, initial_step: f64, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { initial_step_size_sa: initial_step + 0.1, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { initial_step_size_sa: initial_step * 0.1, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}


fn get_best_n_ls_change(base_value: f64, current_best: usize, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { n_best_ls: current_best + 1, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { n_best_ls: current_best - 1, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_best_n_sa_change(base_value: f64, current_best: usize, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { n_best_sa: current_best + 1, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { n_best_sa: current_best - 1, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_min_temp_change(base_value: f64, current_temp: f64, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { min_temp: current_temp + 10f64, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { min_temp: current_temp - 10f64, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_max_iter_ls_change(base_value: f64, current_iter: u64, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { max_iterations_ls: current_iter + 500, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { max_iterations_ls: current_iter - 500, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_max_iter_sa_change(base_value: f64, current_iter: u64, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { max_iterations_sa: current_iter + 500, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { max_iterations_sa: current_iter - 500, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_anneačing_schedule_change(base_value: f64, current_schedule: f64, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { annealing_schedule: current_schedule + 0.02f64, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { annealing_schedule: current_schedule - 0.02f64, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

fn get_initial_temp_change(base_value: f64, initial_temp: u8, problem: &mut BBOBProblem) -> OptionValue {
    let mut res = run_sa(problem, SAOptions { initial_temperature: initial_temp + 5, ..SAOptions::default() });
    let mut direction = Direction::Negative;

    let pos_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    res = run_sa(problem, SAOptions { initial_temperature: initial_temp - 5, ..SAOptions::default() });

    let neg_val = match res {
        Ok(min) => min.value,
        Err(_) => panic!("Error evaluating SA")
    };

    if base_value - pos_val > base_value - neg_val {
        direction = Direction::Positive
    }

    let option_diff = f64::max(base_value - pos_val, base_value - neg_val);
    OptionValue { option: InitialStepSizeLs, option_diff, direction }
}

#[derive(Clone,Copy)]
enum Direction {
    Positive,
    Negative,
}

impl Direction {
    pub fn name(&self) -> &str {
        match self {
            Direction::Positive => "Positive",
            Direction::Negative => "Negative"
        }
    }
}

#[derive(Copy, Clone)]
enum Option {
    InitialTemperature,
    AnnealingSchedule,
    MaxIterationsSa,
    MaxIterationsLs,
    InitialStepSizeSa,
    InitialStepSizeLs,
    MinTemp,
    BestNSa,
    BestNLs,
}

impl Option {
    pub fn iterator() -> Iter<'static, Option> {
        static OPTIONS: [Option; 9] = [InitialTemperature, AnnealingSchedule, MaxIterationsSa,
            MaxIterationsLs, InitialStepSizeSa, InitialStepSizeLs, MinTemp, BestNSa, BestNLs];
        OPTIONS.iter()
    }

    pub fn name(&self) -> &str {
        match self {
            InitialTemperature => "InitialTemperature",
            AnnealingSchedule => "AnnealingSchedule",
            MaxIterationsSa => "MaxIterationsSa",
            MaxIterationsLs => "MaxIterationsLs",
            InitialStepSizeSa => "InitialStepSizeSa",
            InitialStepSizeLs => "InitialStepSizeLs",
            MinTemp => "MinTemp",
            BestNSa => "BestNSa",
            BestNLs => "BestNLs"
        }
    }
}

#[derive(Clone, Copy)]
struct OptionValue {
    pub option: Option,
    pub option_diff: f64,
    pub direction: Direction,
}
