use std::marker::PhantomData;

use pretty::RcDoc;

use crate::{
    fact::traits::{EDBFact, IDBFact},
    pretty::Pretty,
    ram::operation::Operation,
    relation::Relation,
};

#[derive(Debug)]
pub(crate) struct Insert<EF, IF, ER, IR>
where
    EF: EDBFact,
    IF: IDBFact,
    ER: for<'a> Relation<'a, EF>,
    IR: for<'a> Relation<'a, IF>,
{
    operation: Operation<EF, IF, ER, IR>,
    // Whether the insertion is for a ground atom with all constant columns.
    // I don't love this, but it enables us to ensure ground facts are only inserted
    // into the delta relation once.
    is_ground: bool,
    _marker: PhantomData<(EF, IF, ER, IR)>,
}

impl<EF, IF, ER, IR> Insert<EF, IF, ER, IR>
where
    EF: EDBFact,
    IF: IDBFact,
    ER: for<'a> Relation<'a, EF>,
    IR: for<'a> Relation<'a, IF>,
{
    pub(crate) fn new(operation: Operation<EF, IF, ER, IR>, is_ground: bool) -> Self {
        Self {
            operation,
            is_ground,
            _marker: PhantomData::default(),
        }
    }

    pub(crate) fn operation(&self) -> &Operation<EF, IF, ER, IR> {
        &self.operation
    }

    pub(crate) fn is_ground(&self) -> bool {
        self.is_ground
    }
}

impl<EF, IF, ER, IR> Pretty for Insert<EF, IF, ER, IR>
where
    EF: EDBFact,
    IF: IDBFact,
    ER: for<'a> Relation<'a, EF>,
    IR: for<'a> Relation<'a, IF>,
{
    fn to_doc(&self) -> RcDoc<'_, ()> {
        RcDoc::text("insert").append(
            RcDoc::hardline()
                .append(self.operation().to_doc())
                .nest(2)
                .group(),
        )
    }
}
