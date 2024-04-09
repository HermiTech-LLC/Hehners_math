extern crate cpython;

use cpython::{PyResult, Python, PyObject, py_module_initializer, PyClone, PythonObject};
use std::f64::{self, consts::PI};

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

    pub fn ln(&self) -> Option<UnifiedNumber> {
        if self.value <= 0.0 {
            None
        } else {
            let value = self.value.ln();
            let uncertainty = self.uncertainty / self.value;
            Some(UnifiedNumber {
                value,
                uncertainty,
                symbolic: format!("ln({})", self.symbolic),
            })
        }
    }

    pub fn exp(&self) -> UnifiedNumber {
        let value = self.value.exp();
        let uncertainty = self.uncertainty * value;
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("exp({})", self.symbolic),
        }
    }

    pub fn sinh(&self) -> UnifiedNumber {
        let value = self.value.sinh();
        let uncertainty = self.uncertainty * self.value.cosh();
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("sinh({})", self.symbolic),
        }
    }

    pub fn cosh(&self) -> UnifiedNumber {
        let value = self.value.cosh();
        let uncertainty = self.uncertainty * self.value.sinh();
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("cosh({})", self.symbolic),
        }
    }

    pub fn tanh(&self) -> UnifiedNumber {
        let value = self.value.tanh();
        let uncertainty = self.uncertainty / self.value.cosh().powi(2);
        UnifiedNumber {
            value,
            uncertainty,
            symbolic: format!("tanh({})", self.symbolic),
        }
    }
}
pub struct UnifiedVector {
    elements: Vec<UnifiedNumber>,
}

impl UnifiedVector {
    pub fn new(elements: Vec<UnifiedNumber>) -> Self {
        UnifiedVector { elements }
    }

    pub fn add(&self, other: &UnifiedVector) -> Option<UnifiedVector> {
        if self.elements.len() != other.elements.len() {
            None
        } else {
            self.elements.iter().zip(other.elements.iter()).map(|(a, b)| a.add(b)).collect::<Option<Vec<_>>>().map(UnifiedVector::new)
        }
    }

    pub fn subtract(&self, other: &UnifiedVector) -> Option<UnifiedVector> {
        if self.elements.len() != other.elements.len() {
            None
        } else {
            self.elements.iter().zip(other.elements.iter()).map(|(a, b)| a.subtract(b)).collect::<Option<Vec<_>>>().map(UnifiedVector::new)
        }
    }

    // Other vector operations can be added here
}

pub struct UnifiedMatrix {
    rows: Vec<UnifiedVector>,
}

impl UnifiedMatrix {
    pub fn new(rows: Vec<UnifiedVector>) -> Self {
        UnifiedMatrix { rows }
    }

    pub fn add(&self, other: &UnifiedMatrix) -> Option<UnifiedMatrix> {
        if self.rows.len() != other.rows.len() {
            None
        } else {
            self.rows.iter().zip(other.rows.iter()).map(|(a, b)| a.add(b)).collect::<Option<Vec<_>>>().map(UnifiedMatrix::new)
        }
    }

    pub fn subtract(&self, other: &UnifiedMatrix) -> Option<UnifiedMatrix> {
        if self.rows.len() != other.rows.len() {
            None
        } else {
            self.rows.iter().zip(other.rows.iter()).map(|(a, b)| a.subtract(b)).collect::<Option<Vec<_>>>().map(UnifiedMatrix::new)
        }
    }

    // Other matrix operations can be added here
}

fn main() {
    // Example usage of UnifiedNumber, UnifiedVector, UnifiedMatrix
    let u_num1 = UnifiedNumber::new(2.0, 0.1);
    let u_num2 = UnifiedNumber::new(3.0, 0.2);
    let u_num3 = UnifiedNumber::new(4.0, 0.3);
    let u_vector1 = UnifiedVector::new(vec![u_num1, u_num2]);
    let u_vector2 = UnifiedVector::new(vec![u_num2, u_num3]);

    match u_vector1.add(&u_vector2) {
        Some(result) => println!("Vector addition result: {:?}", result),
        None => println!("Vector addition failed due to size mismatch"),
    }

    // Further examples and operations can be implemented here
}
