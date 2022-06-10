use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct EmpArgs {
    /// Enter the Employee data file
    #[clap(short, long)]
    pub empdatafile: String,
    /// Enter the Department data file
    #[clap(short, long)]
    pub deptdatafile: String,
    /// Enter the Salary data file
    #[clap(short, long)]
    pub salarydatafile: String,
    /// Enter the Leave data file
    #[clap(short, long)]
    pub leavedatafile: String,
    /// Enter the Final Summary data file
    #[clap(short, long)]
    pub finalsummarydatafile: String,
}
