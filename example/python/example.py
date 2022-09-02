import sys
from functools import wraps

import cro3n

collected_example_cases = []


def example_case(fn):
    global collected_example_cases
    collected_example_cases.append(fn.__name__)

    @wraps(fn)
    def _(*args, **kwargs):
        return fn(*args, **kwargs)

    return _


class CronExprCase:
    basic_ok = "0 1 21 * * ? *"
    basic_err = "0 1 25 * * ? *"


@example_case
def example_cro3n_check_cron_expression():
    print("CronExprCase.basic_ok", cro3n.check_cron_expression(CronExprCase.basic_ok))
    print("CronExprCase.basic_err", cro3n.check_cron_expression(CronExprCase.basic_err))


@example_case
def example_cro3n_CronExpr():
    print(cro3n.CronExpr(CronExprCase.basic_ok))
    try:
        cro3n.CronExpr(CronExprCase.basic_err)
    except ValueError as e:
        print(e)
    print(cro3n.CronExpr(CronExprCase.basic_ok).next_n_upcoming_time_literals(5))
    print(cro3n.CronExpr(CronExprCase.basic_ok).source == CronExprCase.basic_ok)

def main():
    global collected_example_cases
    if len(sys.argv) == 1:
        print("Example cases:")
        for ec in collected_example_cases:
            print(f"\t{ec}")
    else:
        globals()[sys.argv[1]]()


if __name__ == "__main__":
    main()
