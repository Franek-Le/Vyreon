

pub struct BlockStmt {
  body: Vec<Stmt>
}

impl BlockStmt {
  pub fn stmt() {}
}

pub struct ExpressionStmt {
  expression: Expr,
}

impl ExpressionStmt {
  pub fn stmt() {}
}

pub struct VarDeclStmt {
  variableName: String,
  isConstant: Bool,
  assignedValue: Expr,
  expclicitType: Type,
}

impl VarDeclStmt {
  pub fn stmt() {}
}