use alloc::{borrow::ToOwned, string::String, vec::Vec};
use getargs::{Opt, Options};
use jrinx_multitask::{spawn, yield_now};
use spin::Once;

static BOOTARGS: Once<String> = Once::new();

pub(super) fn set(bootargs: &str) {
    BOOTARGS
        .try_call_once::<_, ()>(|| Ok(bootargs.to_owned()))
        .unwrap();
}

pub async fn execute() {
    if let Some(bootargs) = BOOTARGS.get() {
        let args = bootargs
            .split_whitespace()
            .map(|s| s.to_owned())
            .collect::<Vec<_>>();
        let mut opts = Options::new(args.iter().map(String::as_str));

        info!("bootargs: {}", bootargs);

        while let Some(opt) = opts.next_opt().unwrap() {
            match opt {
                Opt::Short('h') | Opt::Long("help") => help().await,

                Opt::Short('t') | Opt::Long("test") => {
                    test(match opts.value() {
                        Ok(opt) => opt,
                        _ => {
                            panic!("missing argument for option: {opt}, try '-t/--test help' for more information");
                        }
                    }).await;
                }

                Opt::Short(_) | Opt::Long(_) => panic!("unrecognized option: {}", opt),
            };
        }
    }
}

async fn help() {
    info!("boot arguments:");
    info!("   -t, --test <test>    Run the specified test");
    info!("   -h, --help           Display this information");
}

async fn test(arg: &str) {
    if arg == "help" {
        info!("all available tests:");
        let mut all_tests = jrinx_testdef::all().collect::<Vec<_>>();
        all_tests.sort();
        all_tests.iter().for_each(|test| info!("- {test}"));
    } else {
        let test = arg;
        let (name, func) =
            jrinx_testdef::find(test).unwrap_or_else(|| panic!("unrecognized test case: {}", test));
        info!("test case {} begin", name);
        spawn!(async move {
            func();
        });
        yield_now!();
        info!("test case {} end", name);
    }
}
