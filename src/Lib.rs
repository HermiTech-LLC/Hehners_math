extern crate cpython;

use cpython::{PyResult, Python, PyObject, py_module_initializer, PyClone, PythonObject};
use std::f64::consts::PI;

pub struct UnifiedNumber {
    value: f64,
    uncertainty: f64,
    symbolic: String,
}

impl UnifiedNumber {
    pub fn new(value: f64, uncertainty: f64) -> Self {
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("{}±{}", value, uncertainty),
        }
    }

    pub fn add(&self, other: &UnifiedNumber) -> UnifiedNumber {
        UnifiedNumber {
            value: self.value + other.value,
            uncertainty: (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt(),
            symbolic: format!("({}) + ({})", self.symbolic, other.symbolic),
        }
    }

    pub fn subtract(&self, other: &UnifiedNumber) -> UnifiedNumber {
        UnifiedNumber {
            value: self.value - other.value,
            uncertainty: (self.uncertainty.powi(2) + other.uncertainty.powi(2)).sqrt(),
            symbolic: format!("({}) - ({})", self.symbolic, other.symbolic),
        }
    }

    pub fn multiply(&self, other: &UnifiedNumber) -> UnifiedNumber {
        let value = self.value * other.value;
        let rel_uncertainty = (self.uncertainty / self.value).powi(2) + (other.uncertainty / other.value).powi(2);
        UnifiedNumber {
            value,
            uncertainty: value * rel_uncertainty.sqrt(),
            symbolic: format!("({}) * ({})", self.symbolic, other.symbolic),
        }
    }

    pub fn divide(&self, other: &UnifiedNumber) -> Option<UnifiedNumber> {
        if other.value == 0.0 {
            None
        } else {
            let value = self.value / other.value;
            let rel_uncertainty = (self.uncertainty / self.value).powi(2) + (other.uncertainty / other.value).powi(2);
            Some(UnifiedNumber {
                value,
                uncertainty: value * rel_uncertainty.sqrt(),
                symbolic: format!("({}) / ({})", self.symbolic, other.symbolic),
            })
        }
    }

    pub fn power(&self, exponent: f64) -> UnifiedNumber {
        let value = self.value.powf(exponent);
        let uncertainty = self.uncertainty * exponent * value / self.value;
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("({})^{}", self.symbolic, exponent),
        }
    }

    pub fn root(&self, root: f64) -> Option<UnifiedNumber> {
        if self.value < 0.0 && root.round() as i32 % 2 == 0 {
            None
        } else {
            let value = self.value.powf(1.0 / root);
            let uncertainty = self.uncertainty / (root * value.powf(1.0 - 1.0 / root));
            Some(UnifiedNumber {
                value,
                uncertainty,
                symbolic: format!("root({}, {})", self.symbolic, root),
            })
        }
    }

    pub fn log(&self, base: f64) -> Option<UnifiedNumber> {
        if self.value <= 0.0 || base <= 0.0 || base == 1.0 {
            None
        } else {
            let value = self.value.log(base);
            let uncertainty = self.uncertainty / (self.value * base.ln());
            Some(UnifiedNumber {
                value,
                uncertainty,
                symbolic: format!("log({}, {})", self.symbolic, base),
            })
        }
    }

    pub fn sin(&self) -> UnifiedNumber {
        let value = self.value.sin();
        let uncertainty = self.uncertainty * self.value.cos();
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("sin({})", self.symbolic),
        }
    }

    pub fn cos(&self) -> UnifiedNumber {
        let value = self.value.cos();
        let uncertainty = self.uncertainty * self.value.sin().abs();
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("cos({})", self.symbolic),
        }
    }

    pub fn tan(&self) -> Option<UnifiedNumber> {
        let tan_value = self.value.tan();
        if tan_value.is_infinite() || tan_value.is_nan() {
            None
        } else {
            let uncertainty = self.uncertainty / self.value.cos().powi(2);
            Some(UnifiedNumber {
                value: tan_value,
                uncertainty,
                symbolic: format!("tan({})", self.symbolic),
            })
        }
    }

    // Example Hehner's algebra integration - symbolic representation
    pub fn choice(&self, other: &UnifiedNumber, condition: bool) -> UnifiedNumber {
        if condition {
            UnifiedNumber {
                value: self.value,
                uncertainty: self.uncertainty,
                symbolic: format!("({}) if condition else [skipped]", self.symbolic),
            }
        } else {
            UnifiedNumber {
                value: other.value,
                uncertainty: other.uncertainty,
                symbolic: format!("[skipped] if condition else ({})", other.symbolic),
            }
        }
    }

    pub fn display(&self) -> String {
        format!("Value: {}, Uncertainty: {}, Symbolic: {}", self.value, self.uncertainty, self.symbolic)
    }
}

// Python module initialization and Python class definitions remain unchanged

fn main() {
    let num1 = UnifiedNumber::new(5.0, 0.1);
    let num2 = UnifiedNumber::new(3.0, 0.2);
    let result = num1.add(&num2);
    println!("{}", result.display());
}
