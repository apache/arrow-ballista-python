// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use crate::executor::PyExecutor;
use crate::scheduler::PyScheduler;
#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;
use pyo3::prelude::*;

#[allow(clippy::borrow_deref_ref)]
mod context;
#[allow(clippy::borrow_deref_ref)]
mod dataframe;
#[allow(clippy::borrow_deref_ref)]
mod datatype;
pub mod errors;
#[allow(clippy::borrow_deref_ref)]
mod executor;
#[allow(clippy::borrow_deref_ref)]
mod expression;
#[allow(clippy::borrow_deref_ref)]
mod functions;
#[allow(clippy::borrow_deref_ref)]
mod scheduler;
#[allow(clippy::borrow_deref_ref)]
mod udaf;
#[allow(clippy::borrow_deref_ref)]
mod udf;
pub mod utils;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Low-level DataFusion internal package.
///
/// The higher-level public API is defined in pure python files under the
/// datafusion directory.
#[pymodule]
fn _internal(py: Python, m: &PyModule) -> PyResult<()> {
    // Register the python classes
    m.add_class::<context::PyBallistaContext>()?;
    m.add_class::<dataframe::PyDataFrame>()?;
    m.add_class::<expression::PyExpr>()?;
    m.add_class::<udf::PyScalarUDF>()?;
    m.add_class::<udaf::PyAggregateUDF>()?;
    m.add_class::<PyExecutor>()?;
    m.add_class::<PyScheduler>()?;

    // Register the functions as a submodule
    let funcs = PyModule::new(py, "functions")?;
    functions::init_module(funcs)?;
    m.add_submodule(funcs)?;

    Ok(())
}
