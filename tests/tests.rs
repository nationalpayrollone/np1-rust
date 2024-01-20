// National Payroll One - Canadian Payroll Software Helper
// Copyright (C) 2024 National Payroll One

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use np1_rust::{glossary, annual_taxable_income, annual_taxable_income_commission, pay_periods};

#[test]
fn test_annual_taxable_income() {
    // TODO: Replace with an actual test of annual taxable income
    assert_eq!(annual_taxable_income(1, 2, 3, 4, 5, 6, 7, 8), -31);
}

#[test]
fn test_annual_taxable_income_commission() {
    // TODO: Replace with an actual test of annual taxable income commission
    assert_eq!(annual_taxable_income_commission(1, 2, 3, 4, 5, 6, 7, 8), -34);
}

#[test]
fn test_glossary() {
    assert_eq!(glossary("A"), "Annual taxable income");
    assert_eq!(glossary("YTD"), "Year-to-date, not including current pay period");
    assert_eq!(glossary("ZZZ"), "Invalid factor provided. See https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan/t4127-jan-payroll-deductions-formulas-computer-programs.html#toc30 for more details.");
}

#[test]
fn test_pay_periods() {
    assert_eq!(pay_periods("Monthly", None), 12);
    assert_eq!(pay_periods("Semi-monthly", Some::<u16>(365)), 24);
    assert_eq!(pay_periods("Custom", None), 1);
    assert_eq!(pay_periods("Custom", Some::<u16>(365)), 365);
}
