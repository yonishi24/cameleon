use super::{
    elem_type::{IntegerRepresentation, NamedValue, Slope},
    formula::{Expr, Formula},
    interface::{IBoolean, IFloat, IInteger, IncrementMode},
    node_base::{NodeAttributeBase, NodeBase, NodeElementBase},
    store::{CacheStore, NodeId, NodeStore, ValueStore},
    utils, Device, GenApiError, GenApiResult, ValueCtxt,
};

#[derive(Debug, Clone)]
pub struct IntConverterNode {
    pub(crate) attr_base: NodeAttributeBase,
    pub(crate) elem_base: NodeElementBase,

    pub(crate) streamable: bool,
    pub(crate) p_variables: Vec<NamedValue<NodeId>>,
    pub(crate) constants: Vec<NamedValue<i64>>,
    pub(crate) expressions: Vec<NamedValue<Expr>>,
    pub(crate) formula_to: Formula,
    pub(crate) formula_from: Formula,
    pub(crate) p_value: NodeId,
    pub(crate) unit: Option<String>,
    pub(crate) representation: IntegerRepresentation,
    pub(crate) slope: Slope,
}

impl IntConverterNode {
    #[must_use]
    pub fn node_base(&self) -> NodeBase<'_> {
        NodeBase::new(&self.attr_base, &self.elem_base)
    }

    #[must_use]
    pub fn streamable(&self) -> bool {
        self.streamable
    }

    #[must_use]
    pub fn p_variables(&self) -> &[NamedValue<NodeId>] {
        &self.p_variables
    }

    #[must_use]
    pub fn constants(&self) -> &[NamedValue<i64>] {
        &self.constants
    }

    #[must_use]
    pub fn expressions(&self) -> &[NamedValue<Expr>] {
        &self.expressions
    }

    #[must_use]
    pub fn formula_to(&self) -> &Formula {
        &self.formula_to
    }

    #[must_use]
    pub fn formula_from(&self) -> &Formula {
        &self.formula_from
    }

    #[must_use]
    pub fn p_value(&self) -> NodeId {
        self.p_value
    }

    #[must_use]
    pub fn unit_elem(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    #[must_use]
    pub fn representation_elem(&self) -> IntegerRepresentation {
        self.representation
    }

    #[must_use]
    pub fn slope(&self) -> Slope {
        self.slope
    }
}

impl IInteger for IntConverterNode {
    fn value<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        self.elem_base.verify_is_readable(device, store, cx)?;

        let mut collector =
            utils::FormulaEnvCollector::new(&self.p_variables, &self.constants, &self.expressions);
        collector.insert("FROM", self.p_value(), device, store, cx)?;
        let var_env = collector.collect(device, store, cx)?;

        let eval_result = self.formula_from.eval(&var_env);
        Ok(eval_result.as_integer())
    }

    fn set_value<T: ValueStore, U: CacheStore>(
        &self,
        value: i64,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        self.elem_base.verify_is_writable(device, store, cx)?;
        cx.invalidate_cache_by(self.node_base().id());

        let mut collector =
            utils::FormulaEnvCollector::new(&self.p_variables, &self.constants, &self.expressions);
        collector.insert_imm("TO", value);
        let var_env = collector.collect(device, store, cx)?;

        let eval_result = self.formula_to.eval(&var_env);
        let nid = self.p_value();
        if let Some(node) = nid.as_iinteger_kind(store) {
            node.set_value(eval_result.as_integer(), device, store, cx)?;
        } else if let Some(node) = nid.as_ifloat_kind(store) {
            node.set_value(eval_result.as_float(), device, store, cx)?;
        } else if let Some(node) = nid.as_iboolean_kind(store) {
            node.set_value(eval_result.as_bool(), device, store, cx)?;
        } else {
            return Err(GenApiError::InvalidNode("`pValue` elem of `IntConverterNode` doesn't implement `IInteger`/`IFloat`/`IBoolean`".into()));
        }

        Ok(())
    }

    fn min<T: ValueStore, U: CacheStore>(
        &self,
        _: &mut impl Device,
        _: &impl NodeStore,
        _: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        Ok(i64::MIN)
    }

    fn max<T: ValueStore, U: CacheStore>(
        &self,
        _: &mut impl Device,
        _: &impl NodeStore,
        _: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<i64> {
        Ok(i64::MAX)
    }

    fn inc_mode(&self, _: &impl NodeStore) -> GenApiResult<Option<IncrementMode>> {
        Ok(None)
    }

    fn inc<T: ValueStore, U: CacheStore>(
        &self,
        _: &mut impl Device,
        _: &impl NodeStore,
        _: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<Option<i64>> {
        Ok(None)
    }

    fn valid_value_set(&self, _: &impl NodeStore) -> &[i64] {
        &[]
    }

    fn representation(&self, _: &impl NodeStore) -> IntegerRepresentation {
        self.representation
    }

    fn unit(&self, _: &impl NodeStore) -> Option<&str> {
        todo!()
    }

    fn set_min<T: ValueStore, U: CacheStore>(
        &self,
        _: i64,
        _: &mut impl Device,
        _: &impl NodeStore,
        _: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        Err(GenApiError::AccessDenied(
            "can't set value to min elem of `IntConverterNode`".into(),
        ))
    }

    fn set_max<T: ValueStore, U: CacheStore>(
        &self,
        _: i64,
        _: &mut impl Device,
        _: &impl NodeStore,
        _: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<()> {
        Err(GenApiError::AccessDenied(
            "can't set value to max elem of `IntConverterNode`".into(),
        ))
    }

    fn is_readable<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<bool> {
        self.elem_base.is_readable(device, store, cx)
    }

    fn is_writable<T: ValueStore, U: CacheStore>(
        &self,
        device: &mut impl Device,
        store: &impl NodeStore,
        cx: &mut ValueCtxt<T, U>,
    ) -> GenApiResult<bool> {
        self.elem_base.is_writable(device, store, cx)
    }
}
