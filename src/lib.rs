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

use time::weeks_in_year;

#[allow(clippy::too_many_arguments)]
pub fn annual_taxable_income(p: isize, i: isize, f: isize, f2: isize, f5a: isize, u1: isize, hd: isize, f1: isize) -> isize {
    (p * (i - f - f2 - f5a - u1)) - hd - f1
}

#[allow(clippy::too_many_arguments)]
pub fn annual_taxable_income_commission(i1: isize, f: isize, f2: isize, f5a: isize, u1: isize, hd: isize, f1: isize, e: isize) -> isize {
    i1 - f - f2 - f5a - u1 - hd - f1 - e
}

#[allow(clippy::too_many_arguments)]
pub fn annual_basic_federal_tax(r: isize, a: isize, k: isize, k1: isize, k2: isize, k3: isize, k4: isize) -> isize {
    let fit = (r * a) - k - k1 - k2 - k3 - k4;
    if fit < 0 {
        return 0;
    }
    fit
}

#[allow(clippy::too_many_arguments)]
pub fn annual_basic_federal_tax_quebec(r: isize, a: isize, k: isize, k1: isize, k2q: isize, k3: isize, k4: isize) -> isize {
    let fit = (r * a) - k - k1 - k2q - k3 - k4;
    if fit < 0 {
        return 0;
    }
    fit
}

pub fn annual_federal_tax_payable(t3: isize, p: isize, lcf: isize) -> isize {
    let ftp = t3 - (p * lcf);
    if ftp < 0 {
        return 0;
    }
    ftp
}

pub fn annual_federal_tax_payable_quebec(t3: isize, p: isize, lcf: isize) -> isize {
    let ftp = t3 - (p * lcf) - (0.165 as isize * t3);
    if ftp < 0 {
        return 0;
    }
    ftp
}

pub fn annual_federal_tax_payable_outside(t3: isize, p: isize, lcf: isize) -> isize {
    let ftp = t3 + (0.48 as isize * t3) - (p * lcf);
    if ftp < 0 {
        return 0;
    }
    ftp
}

#[allow(clippy::too_many_arguments)]
pub fn annual_basic_provincial_tax(v: isize, a: isize, kp: isize, k1p: isize, k2p: isize, k3p: isize, k4p: isize) -> isize {
    let pit =(v * a) - kp - k1p - k2p - k3p - k4p;
    if pit < 0 {
        return 0;
    }
    pit
}

#[allow(clippy::too_many_arguments)]
pub fn annual_basic_terrirotial_tax(v: isize, a: isize, kp: isize, k1p: isize, k2p: isize, k3p: isize, k4p: isize) -> isize {
    annual_basic_provincial_tax(v, a, kp, k1p, k2p, k3p, k4p)
}

#[allow(clippy::too_many_arguments)]
pub fn annual_provincial_tax_deduction(t4: isize, v1: isize, v2: isize, s: isize, p: isize, lcp: isize) -> isize {
    let ptd = t4 + v1 + v2 - s - (p * lcp);
    if ptd < 0 {
        return 0;
    }
    ptd
}

#[allow(clippy::too_many_arguments)]
pub fn annual_territorial_tax_deduction(t4: isize, v1: isize, v2: isize, s: isize, p: isize, lcp: isize) -> isize {
    annual_provincial_tax_deduction(t4, v1, v2, s, p, lcp)
}

pub fn edtimated_federal_and_provincial_tax_deductions(t1: isize, t2: isize, p: isize, l: isize) -> isize {
    ((t1 + t2) / p) + l
}

pub fn edtimated_federal_and_territorial_tax_deductions(t1: isize, t2: isize, p: isize, l: isize) -> isize {
    edtimated_federal_and_provincial_tax_deductions(t1, t2, p, l)
}

pub fn edtimated_federal_and_quebec_tax_deductions(t1: isize, p: isize, l: isize) -> isize {
    edtimated_federal_and_provincial_tax_deductions(t1, 0, p, l)
}

pub fn pay_periods(period: &str, custom: Option<u16>) -> u16 {
    const YEAR: i32 = 2024;
    match period {
        "Weekly"=>weeks_in_year(YEAR) as u16,
        "Biweekly"=>(weeks_in_year(YEAR) as f32 / 2.0).round() as u16,
        "Semi-monthly"=>24,
        "Monthly"=>12,
        _=> custom.unwrap_or(1)
    }
}

// Table 3.1 Glossary, Chapter 3
pub fn glossary(factor: &str) -> &str {
    match factor {
        "A"=>     "Annual taxable income",
        "B"=>     "Gross bonus, retroactive pay increase, vacation pay when vacation is not taken, accumulated overtime payment or other non-periodic payment",
        "B1"=>    "Gross bonuses, retroactive pay increases, vacation pay when vacation is not taken, accumulated overtime payments or other non-periodic payments year-to-date (before the pay period)",
        "BPAF"=>  "Federal Basic Personal Amount",
        "BPANS"=> "Basic Personal Amount for Nova Scotia",
        "BPAYT"=> "Basic Personal Amount for Yukon",
        "C"=>     "Canada (or Quebec) Pension Plan contributions for the pay period",
        "C2"=>    "Second additional Canada (or Quebec) Pension Plan contributions for the pay period",
        "CEA"=>   "Canada Employment Amount, a non-refundable tax credit used in the calculation for K4 and K4P",
        "D"=>     "Employee’s year-to-date (before the pay period) Canada Pension Plan contribution with the employer",
        "DQ"=>    "Employee’s year-to-date (before the pay period) Quebec Pension Plan contribution with the employer",
        "D1"=>    "Employee’s year-to-date (before the pay period) employment insurance premium with the employer",
        "D2"=>    "Employee’s year-to-date (before the pay period) second additional Canada Pension Plan contribution with the employer",
        "D2Q"=>   "Employee’s year-to-date (before the pay period) second additional Quebec Pension Plan contribution with the employer ",
        "E"=>     "Total commission expenses deductions reported on Form TD1X",
        "EI"=>    "Employment insurance premiums for the pay period",
        "F"=>     "Payroll deductions for the pay period for employee contributions to a registered pension plan (RPP) for current and past services, a registered retirement savings plan (RRSP), to a pooled registered pension plan (PRPP), or a retirement compensation arrangement (RCA). For tax deduction purposes, employers can deduct amounts contributed to an RPP, RRSP, PRPP, or RCA by or on behalf of an employee to determine the employee's taxable income",
        "F1"=>    "Annual deductions such as child care expenses and support payments requested by an employee or pensioner and authorized by a tax services office or tax centre",
        "F2"=>    "Alimony or maintenance payments required by a legal document dated before May 1, 1997, to be payroll-deducted authorized by a tax services office or tax centre",
        "F3"=>    "Employee registered pension plan or registered retirement savings plan contributions deducted from the current non-periodic payment. You can also use this field or design another to apply other tax-deductible amounts to the non-periodic payment, such as union dues",
        "F4"=>    "Employee registered pension plan or registered retirement savings plan contributions deducted from the year-to-date non-periodic payments. You can also use this field or design another to apply other tax-deductible amounts to the non-periodic payment, such as union dues",
        "F5"=>    "Deductions for Canada Pension Plan additional contributions for the pay period",
        "F5A"=>   "Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the periodic income",
        "F5B"=>   "Deductions for Canada (or Quebec) Pension Plan additional contributions for the pay period deducted from the non-periodic payment",
        "F5Q"=>   "Deductions for Quebec Pension Plan additional contributions for the pay period",
        "G"=>     "Gross commission amount including gross salary at the time of payment, plus any taxable benefits for commission-remunerated employees who have filled out Form TD1X. When an employee has not filed Form TD1X, tax is calculated the regular way",
        "HD"=>    "Annual deduction for living in a prescribed zone, as shown on Form TD1",
        "I"=>     "Gross remuneration for the pay period. This includes overtime earned and paid in the same pay period, pension income, qualified pension income, and taxable benefits, but does not include bonuses, retroactive pay increases, or other non-periodic payments",
        "I1"=>    "Total remuneration for the year reported on Form TD1X. This includes commission payments, salary (where applicable), non-periodic payments, and taxable benefits",
        "IE"=>    "Insurable earnings for the pay period including insurable taxable benefits, bonuses, and retroactive pay increases",
        "K"=>     "Federal constant. The constant is the tax overcharged when applying the 20.5%, 26%, 29%, and 33% rates to the annual taxable income A",
        "KP"=>    "Provincial or territorial constant",
        "K1"=>    "Federal non-refundable personal tax credit (the lowest federal tax rate is used to calculate this credit)",
        "K1P"=>   "Provincial or territorial non-refundable personal tax credit (the lowest tax rate of the province or territory is used to calculate this credit)",
        "K2"=>    "Base Canada Pension Plan contributions and employment insurance premiums federal tax credits for the year (the lowest federal tax rate is used to calculate this credit). Note: If an employee has already contributed the maximum CPP and EI, for the year with the employer, use the maximum base CPP contribution and the maximum EI premium to calculate the credit for the rest of the year. If, during the pay period in which the employee reaches the maximum, the CPP and  EI, when annualized, is less than the annual maximum, use the maximum base CPP contribution and the maximum EI premium in that pay period",
        "K2P"=>   "Provincial or territorial base Canada Pension Plan contributions and employment insurance premiums tax credits for the year (the lowest provincial or territorial tax rate is used to calculate this credit). If an employee reaches the maximum CPP or EI for the year with an employer, the instructions in the note for the K2 factor also apply to the K2P factor. For employees paid by commission, use the federal K2 formula for commissions and replace the lowest federal rate in the K2 formula with the lowest provincial or territorial tax rate",
        "K2Q"=>   "Base Quebec Pension Plan contributions, employment insurance premiums, and Quebec Parental Insurance Plan premiums federal tax credits for the year (the lowest federal tax rate is used to calculate this credit)",
        "K2R"=>   "Base Canada Pension Plan contributions and employment insurance premiums tax credits for the year (the lowest federal tax rate is used to calculate this credit), for employees that are transferred from Quebec to a location outside Quebec. The term “location outside Quebec” refers to a province or territory outside Quebec",
        "K2RQ"=>  "Base Quebec Pension Plan contributions, employment insurance premiums, and Quebec Parental Insurance Plan premiums federal tax credits for the year (the lowest federal tax rate is used to calculate this credit), for employees that are transferred to Quebec from a location outside Quebec. The term “location outside Quebec” refers to a province or territory outside Quebec",
        "K3"=>    "Other federal non-refundable tax credits (such as medical expenses and charitable donations) authorized by a tax services office or tax centre",
        "K3P"=>   "	Other provincial or territorial non-refundable tax credits (such as medical expenses and charitable donations) authorized by a tax services office or tax centre",
        "K4"=>    "	Federal non-refundable tax credit calculated using the Canada employment amount (the lowest federal tax rate is used to calculate this credit)",
        "K4P"=>   "Territorial non-refundable tax credit calculated using the provincial or territorial Canada employment amount (the lowest territorial tax rate is used to calculate this credit)",
        "L"=>     "Additional tax deductions for the pay period requested by the employee or pensioner as shown on Form TD1",
        "LCF"=>   "Federal labour-sponsored funds tax credit",
        "LCP"=>   "Provincial or territorial labour-sponsored funds tax credit",
        "M"=>     "Accumulated federal and provincial or territorial tax deductions (if any) to the end of the last pay period",
        "M1"=>    "Year-to-date tax deducted on all payments included in B1",
        "N"=>     "The number of days in the current year since the last payment. The minimum basic exemption amount of $67.30 is included in the formula in line with CPP legislation",
        "NI"=>    "Net income for the year from the employer",
        "P"=>     "The number of pay periods in the year",
        "PI"=>    "Pensionable earnings for the pay period, or the gross income plus any taxable benefits for the pay period, including bonuses and retroactive pay increases where applicable",
        "PM"=>    "The total number of months during which CPP and/or QPP contributions are required to be deducted (used in the proration of maximum contribution). For detailed information and examples, refer to T4001 Employers’ Guide – Payroll Deductions and Remittances.",
        "PR"=>    "The number of pay periods left in the year (including the current pay period)",
        "R"=>     "Federal tax rate that applies to the annual taxable income A",
        "S"=>     "Provincial tax reduction (only applies to Ontario and British Columbia)",
        "S1"=>    "Annualizing factor",
        "S2"=>    "Basic amount used in the calculation of Factor S (only applies to Ontario and British Columbia)",
        "T"=>     "Estimated federal and provincial or territorial tax deductions for the pay period",
        "T1"=>    "Annual federal tax deduction",
        "T2"=>    "Annual provincial or territorial tax deduction (except Quebec)",
        "T3"=>    "Annual basic federal tax",
        "T4"=>    "Annual basic provincial or territorial tax",
        "TB"=>    "Tax deductions, i.e., bonuses or retroactive pay increases, payable now",
        "TC"=>    "\"Total claim amount,\" reported on federal Form TD1. If Form TD1 is not filed by the employee or pensioner, calculate TC using BPAF formula, and for non-resident individuals, TC is $0. If the claim code is E, T = $0. If the province is Ontario, even if the claim code is E, the Ontario Health Premium is payable on annual income over $20,000",
        "TCP"=>   "\"Total claim amount,\" reported on the provincial or territorial Form TD1. If this form is not filed, TCP is the provincial or territorial basic personal amount, refer to Table 8.2. For Nova Scotia and Yukon, use BPANS and BPAYT formulas respectively.",
        "U1"=>    "Union dues for the pay period paid to a trade union, an association of public servants, or dues required under the law of a province to a parity or advisory committee or similar body",
        "V"=>     "Provincial or territorial tax rate for the year (does not apply to Quebec, outside Canada, or in Canada beyond the limits of any province or territory)",
        "V1"=>    "Provincial surtax calculated on the basic provincial tax (only applies to Ontario)",
        "V2"=>    "Additional tax calculated on taxable income (only applies to the Ontario Health Premium)",
        "W"=>     "The greater of year-to-date (before the pay period) pensionable earnings (PIYTD or GYTD) and employee’s Year’s Maximum Pensionable Earnings (YMPE). This is used to calculate Factor C2",
        "Y"=>     "Additional provincial tax reduction amount based on the number of eligible dependents used in the calculation of Factor S (only applies to Ontario)",
        "YAMPE"=> "Year’s Additional Maximum Pensionable Earnings",
        "YMPE"=>  "Year’s Maximum Pensionable Earnings",
        "YTD"=>   "Year-to-date, not including current pay period",
        _=>       "Invalid factor provided. See https://www.canada.ca/en/revenue-agency/services/forms-publications/payroll/t4127-payroll-deductions-formulas/t4127-jan/t4127-jan-payroll-deductions-formulas-computer-programs.html#toc30 for more details."
    }
}
