use crate::database::db_procedures_common::Procedure;
use crate::database::db_procedures_common::data_affect::ReadOnly;
use crate::database::db_procedures_common::tx_affect::No;
use std::marker::PhantomData;

pub mod db_procedures_common;

#[cfg(not(test))]
mod procedures_prod;
#[cfg(test)]
mod procedures_test;




pub fn hello_proc() -> impl Procedure<DataAffect = ReadOnly, TxAffect = No, InputParams = (), OutputParams = ()>{

    #[cfg(test)]
    procedures_test::Proc{
        phantom_data : PhantomData,
        phantom_tx : PhantomData
    };

    #[cfg(not(test))]
    procedures_prod::Proc{
        phantom_data : PhantomData,
        phantom_tx : PhantomData
    }
}
