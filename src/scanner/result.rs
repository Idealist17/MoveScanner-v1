use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};
use ansi_term::Colour;

pub type ModuleName = String;
pub type FunctionName = String;
// 路径+行号
pub type Location = String;


#[derive(Debug, Display, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum Status {
    Pass,
    Wrong,
}

#[derive(Debug, EnumIter, Display, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DetectKind {
    UncheckedReturn,
    Overflow,
    PrecisionLoss,
    InfiniteLoop,
    UnnecessaryTypeConversion,
    UnnecessaryBoolJudgment,
    UnusedConstant,
    UnusedPrivateFunctions,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum FunctionType {
    All,
    Native,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Info,
    Minor,
    Medium,
    Major,
    Critical,
}

pub struct DetectContent {
    pub severity: Severity,
    pub kind: DetectKind,
    pub result: HashMap<ModuleName, Vec<String>>,
}

impl DetectContent {
    pub fn new(severity: Severity, kind: DetectKind) -> Self {
        Self {
            severity,
            kind,
            result: HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Result {
    pub modules_status: HashMap<Status, Vec<String>>,
    pub total_time: usize,
    pub modules: HashMap<ModuleName, ModuleInfo>,
}

impl Result {
    pub fn new(
        modules_status: HashMap<Status, Vec<String>>,
        total_time: usize,
        modules: HashMap<ModuleName, ModuleInfo>,
    ) -> Self {
        Self {
            modules_status,
            total_time,
            modules,
        }
    }

    pub fn empty() -> Self {
        return Self::new(
            HashMap::from([(Status::Pass, Vec::new()), (Status::Wrong, Vec::new())]),
            0,
            HashMap::new(),
        );
    }

    pub fn add_module(&mut self, module_name: ModuleName, module_info: ModuleInfo) {
        self.modules.insert(module_name, module_info);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModuleInfo {
    pub status: Status,
    pub location: Option<Location>,
    pub function_count: HashMap<FunctionType, usize>,
    pub constant_count: usize,
    pub detectors: HashMap<DetectKind, Vec<String>>,
}
impl ModuleInfo {
    pub fn new(
        status: Status,
        location: Option<Location>,
        function_count: HashMap<FunctionType, usize>,
        constant_count: usize,
        detectors: HashMap<DetectKind, Vec<String>>,
    ) -> Self {
        Self {
            status,
            location,
            function_count,
            constant_count,
            detectors,
        }
    }
    pub fn empty() -> Self {
        let function_count = HashMap::from([(FunctionType::All, 0), (FunctionType::Native, 0)]);
        let mut detectors = HashMap::new();
        for detect_kind in DetectKind::iter() {
            detectors.insert(detect_kind, Vec::<String>::new());
        }
        return Self::new(Status::Wrong, None, function_count, 0, detectors);
    }
}

impl std::fmt::Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "\n{}: {:<8} {}: {:<8} time: {:<8} us\n",
            Status::Pass,
            Colour::Green.paint(self.modules_status.get(&Status::Pass).unwrap().len().to_string()),
            Status::Wrong,
            Colour::Red.paint(self.modules_status.get(&Status::Pass).unwrap().len().to_string()),
            Colour::Blue.paint(self.total_time.to_string())
        )?;

        for (module_index, (module_name, module_info)) in self
            .modules_status
            .get(&Status::Wrong)
            .unwrap()
            .iter()
            .map(|module_name| {
                let module_info = self.modules.get(module_name).unwrap();
                (module_name, module_info)
            })
            .enumerate()
        {
            writeln!(f, "no: {}", module_index)?;
            writeln!(f, "module_name: {}", module_name)?;
            if let Some(location) = &module_info.location{
                writeln!(f, "module_location: {}", location)?;
            }
            for (detector_type, values) in module_info.detectors.clone() {
                if values.is_empty() {
                    // writeln!(f, "\n")?;
                    continue;
                }
                write!(f, "{}: ", Colour::Red.paint(detector_type.to_string()))?;
                let values_str = values.iter().join(",");
                match detector_type {
                    DetectKind::UncheckedReturn => {
                        let color_value_str = &values_str
                            .replace("(", "\x1B[33m(")
                            .replace(")", ")\x1B[0m");
                        writeln!(f, "[ {} ]", color_value_str)?;
                    }
                    _ => {
                        writeln!(f, "[ {} ] ", values_str)?;
                    }
                }
            }
            writeln!(f, "\n")?;
        }
        Ok(())
    }
}
