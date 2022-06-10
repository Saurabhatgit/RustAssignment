mod arguments;

use clap::Parser;
use arguments::EmpArgs;
use calamine::{open_workbook, Error, Xlsx, Xls, Reader, RangeDeserializerBuilder,DataType};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use chrono::prelude::*;
use chrono::{DateTime, FixedOffset, TimeZone};

#[derive(Debug)]
pub struct initial_employee_data {
    EmpId: String,
    EmpName: String,
    DeptId: String,
    MobileNo: String,
    Email : String
}

#[derive(Debug)]
pub struct leave_report{
    LeaveId: String,
    LeaveFrom: String,
    LeaveTo: String,
    LeaveType: String
}

#[derive(Debug)]
pub struct Salary_report{
    SalaryId: String,
    SalaryDate: String,
    Salary: String,
}

#[derive(Debug)]
pub struct final_report {
    EmpId: String,
    EmpName: String,
    DeptTitle: String,
    MobileNo: String,
    Email : String,
    SalaryStatus: String,
    OnLeave: String
}

fn read_dept_data_file(deptfilepath: String)->HashMap<String,String> {
    let path = deptfilepath;
    let mut workbook: Xls<_> = open_workbook(path).expect("Cannot open file");
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();
    let mut iter = RangeDeserializerBuilder::new().has_headers(true).from_range(&range).unwrap();
    let mut mapofdept=HashMap::new();
    while let Some(result) = iter.next() {
        let (DeptID, Dept_title, Dept_Strength): (String, String, String) = result.unwrap();
        mapofdept.insert(DeptID, Dept_title);
    }
    return mapofdept;
 }

 fn read_emp_data_file(empfilepath: String) -> HashMap<String,initial_employee_data> {
    let mut mapofemp:HashMap<String,initial_employee_data>=HashMap::new();
    let file = File::open(empfilepath).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        if index==0 {
            continue;
        }
        let line = line.unwrap();
        let tokens:Vec<&str>= line.split("|").collect();
        let EmpId = tokens[0].to_string();
        let EmpName = tokens[1].to_string();
        let DeptId = tokens[2].to_string();
        let MobileNo = tokens[3].to_string();
        let Email = tokens[4].to_string();
        let employee = initial_employee_data{EmpId, EmpName, DeptId, MobileNo,Email};
        mapofemp.insert(tokens[0].to_string(),employee );
    } 
    return mapofemp;
 }

 fn read_salary_data_file(salaryfilepath: String)->HashMap<String,Salary_report> {
    let path = salaryfilepath;
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();
    let mut iter = RangeDeserializerBuilder::new().has_headers(true).from_range(&range).unwrap();
    let mut mapofsalary:HashMap<String, Salary_report >=HashMap::new();
    while let Some(result) = iter.next() {
        let (EmpId, SalaryID, Salarydate, salary): (String, String, String,String) = result.unwrap();
        let SalaryId = SalaryID;
        let SalaryDate = Salarydate;
        let Salary = salary;
        let SalaryStatusReport = Salary_report { SalaryId, SalaryDate, Salary};
        mapofsalary.insert(EmpId, SalaryStatusReport);
    }
    return mapofsalary;
 }

 fn read_leave_data_file(leavefilepath: String)->HashMap<String,leave_report> {
    let path = leavefilepath;
    let mut workbook: Xlsx<_> = open_workbook(path).expect("Cannot open file");
    let range = workbook.worksheet_range("Sheet1").unwrap().unwrap();
    let mut iter = RangeDeserializerBuilder::new().has_headers(true).from_range(&range).unwrap();
    let mut mapofleave:HashMap<String, leave_report >=HashMap::new();
    while let Some(result) = iter.next() {
        let (EmpId, leaveID, leavefrom, leaveto, leavetype): (String, String, String,String,String) = result.unwrap();
        let LeaveId = leaveID;
        let LeaveFrom = leavefrom;
        let LeaveTo = leaveto;
        let LeaveType = leavetype;
        let LeaveStatusReport = leave_report { LeaveId, LeaveFrom, LeaveTo, LeaveType};
        mapofleave.insert(EmpId, LeaveStatusReport);
    }
    return mapofleave;
 }
 fn get_empid_dept_id_pair(empfilepath: String) ->HashMap<String, String> {
    let mut tempmap:HashMap<String, initial_employee_data >=HashMap::new();
    tempmap = read_emp_data_file(empfilepath);
    let mut empid_deptid_pair:HashMap<String, String>=HashMap::new();
    for (key, val) in tempmap.iter(){
        empid_deptid_pair.insert(key.to_string(), val.DeptId.to_string());
    }
    return empid_deptid_pair;
 }

 fn get_final_report(empfilepath: String,deptfilepath: String,salaryfilepath: String,leavefilepath: String) -> HashMap<String,final_report> {
    let localcurrentmonth: DateTime<Local> = Local::now();
    let currentmonnth = localcurrentmonth.format("%b %Y",).to_string();
    let mut mapoffinalreport:HashMap<String, final_report >=HashMap::new();
    let mut mapofemp:HashMap<String,initial_employee_data>=HashMap::new();
    let mut mapofdept:HashMap<String,String>=HashMap::new();
    let mut mapofsalary:HashMap<String, Salary_report >=HashMap::new();
    let mut mapofleave:HashMap<String, leave_report >=HashMap::new();
    let mut empid_deptid_pair:HashMap<String, String>=HashMap::new();
    mapofemp = read_emp_data_file(empfilepath.to_owned());
    mapofdept = read_dept_data_file(deptfilepath);
    mapofsalary = read_salary_data_file(salaryfilepath);
    mapofleave = read_leave_data_file(leavefilepath);
    empid_deptid_pair = get_empid_dept_id_pair(empfilepath);
    for(key,value) in empid_deptid_pair.iter(){
        let employeeid = key;
        let EmployeedeptId = value;
        let DeptTitle = mapofdept.get(EmployeedeptId).unwrap().to_string();
        let empdetail = mapofemp.get(employeeid).unwrap();
        let empsalarydetail = mapofsalary.get(employeeid).unwrap();
        let empleavedetail = mapofleave.get(employeeid).unwrap();
        let mut SalaryStatus = "".to_string();
        if empsalarydetail.Salary!=""{
            let lastsalarymonth = empsalarydetail.SalaryDate.to_string();
            let check = currentmonnth.eq(&lastsalarymonth);
            if check==true {
                SalaryStatus = "Credited".to_string();
            }
            else{
                SalaryStatus = "Not Credited".to_string();
            }
        }
        else {
            SalaryStatus = "Not Credited".to_string();
        }
        let EmpId = empdetail.EmpId.to_string();
        let EmpName = empdetail.EmpName.to_owned();
        let MobileNo = empdetail.MobileNo.to_owned();
        let Email = empdetail.Email.to_owned();
        let employeeleavefrom = empleavedetail.LeaveFrom.to_string();
        let employeeleaveto = empleavedetail.LeaveTo.to_string();
        let mut OnLeave = "".to_string();
        if employeeleavefrom!=""{
        let naiveleavefrom = NaiveDate::parse_from_str(&employeeleavefrom, "%d-%m-%Y").unwrap();
        let naiveleaveto = NaiveDate::parse_from_str(&employeeleaveto, "%d-%m-%Y").unwrap();
        OnLeave = naiveleaveto.signed_duration_since(naiveleavefrom).num_days().to_string();
        }
        else {
            OnLeave = "0".to_string();
        }
        let employee = final_report{EmpId,EmpName,DeptTitle,MobileNo,Email,SalaryStatus,OnLeave};
        mapoffinalreport.insert(key.to_string(), employee);
    }
    return mapoffinalreport;
 }
 fn write_summary(finalsummarypath:String, mapoffinalreport:HashMap<String, final_report >){
    let mut Summary = std::fs::File::create(finalsummarypath).expect("create failed");
    for (key,value) in mapoffinalreport.iter(){
        let ID = value.EmpId.to_owned();
        let name = value.EmpName.to_owned();
        let depttitle = value.DeptTitle.to_owned();
        let mobile = value.MobileNo.to_owned();
        let email = value.Email.to_owned();
        let salarystatus = value.SalaryStatus.to_owned();
        let onleave = value.OnLeave.to_owned();
        let delimiter = "~#~".to_string();
        let v = vec![ID,name,depttitle,mobile,email,salarystatus,onleave];
        let s = v.connect("~#~");
        Summary.write_all(s.as_bytes());
        Summary.write_all("\n".as_bytes());
    }
 }
fn main() {
    let args = EmpArgs::parse();
    let empdatafilepath = args.empdatafile;
    let deptdatafilepath = args.deptdatafile;
    let salarydatafilepath = args.salarydatafile;
    let leavedatafilepath = args.leavedatafile;
    let finalsummarydatafilepath = args.finalsummarydatafile;
    let mut mapoffinalreport:HashMap<String, final_report >=HashMap::new();
    mapoffinalreport =  get_final_report(empdatafilepath,deptdatafilepath,salarydatafilepath,leavedatafilepath);
    write_summary(finalsummarydatafilepath, mapoffinalreport);
}