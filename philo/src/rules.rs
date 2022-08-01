pub struct  Rules
{
        pub nbr_philo:  i32,
        pub t_die:      i32,
        pub t_eat:      i32,
        pub t_sleep:    i32,
        pub max_eat:    i32,
}

pub fn init_rules(args: Vec<String>) -> Rules {
    let mut stru = Rules{nbr_philo: args[1].parse::<i32>().unwrap(),
    t_die: args[2].parse::<i32>().unwrap(),
    t_eat: args[3].parse::<i32>().unwrap(),
    t_sleep: args[4].parse::<i32>().unwrap(),max_eat: -1};
    if args.len() == 6
    {stru.max_eat = args[5].parse::<i32>().unwrap();}
    return stru;
}
