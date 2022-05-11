use linprog::{
    Model,
    Objective,
    Summand,
    Operator,
    Var
};

fn main() {
    println!("max. 3x + 5y\nst.  x + 2y <= 170\n     3y <= 180");
    let mut model = Model::new("Readme example", Objective::Max);
    let mut vars: Vec<Var> = vec![];
    
    vars.push(model.reg_var(3.0));
    vars.push(model.reg_var(5.0));
    
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(2.0, &vars[1])],
        Operator::Le,
        170.0,
    );
    model.reg_constr(
        vec![Summand(1.0, &vars[0]), Summand(1.0, &vars[1])],
        Operator::Le,
        150.0,
    );
    model.reg_constr(
        vec![Summand(0.0, &vars[0]), Summand(3.0, &vars[1])],
        Operator::Le,
        180.0,
    );
    
    model.optimize();
    print!("{}", model);
}
