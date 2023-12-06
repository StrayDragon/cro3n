import pytest

import cro3n


class CronExprCase:
    basic_ok = "0 1 21 * * ? *"
    basic_err = "0 1 25 * * ? *"


def test_check_cron_expression():
    # 1
    assert not cro3n.check_cron_expression(CronExprCase.basic_ok)

    # 2
    assert "failed, due to Error" in cro3n.check_cron_expression(CronExprCase.basic_err)


def test_CronExpr():
    # 1
    assert cro3n.CronExpr(CronExprCase.basic_ok)

    # 2
    with pytest.raises(ValueError, match="failed, due to Error"):
        cro3n.CronExpr(CronExprCase.basic_err)

    # 3
    assert ['2023-12-06 21:01:00 UTC', '2023-12-07 21:01:00 UTC', '2023-12-08 21:01:00 UTC', '2023-12-09 21:01:00 UTC', '2023-12-10 21:01:00 UTC'] == cro3n.CronExpr(CronExprCase.basic_ok).next_n_upcoming_time_literals(5)

    # 4
    assert cro3n.CronExpr(CronExprCase.basic_ok).source == CronExprCase.basic_ok
