use serde::Serialize;

#[derive(Serialize)]
pub struct FeatureOutput {
    name: String,
    uri: String,
    elements: Vec<ElementOutput>,
}

#[derive(Serialize)]
pub struct ElementOutput {
    scenario_type: String,
    line: u16,
    name: String,
    steps: Vec<StepOutput>,
}

#[derive(Serialize)]
pub enum StepKeyword{
    Given,
    When,
    Then
}

#[derive(Serialize)]
pub struct StepOutput {
    keyword: StepKeyword,
    line: u16,
    result: ResultOutput
}

#[derive(Serialize)]
pub enum ResultStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Serialize)]
pub struct ResultOutput {
    status: ResultStatus,
    error_message: Option<String>
}
