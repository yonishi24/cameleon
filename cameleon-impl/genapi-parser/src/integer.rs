use super::{elem_type::*, node_base::*, xml, Parse};

#[derive(Debug, Clone)]
pub struct IntegerNode {
    attr_base: NodeAttributeBase,

    elem_base: NodeElementBase,

    p_invalidators: Vec<String>,

    streamable: bool,

    value_kind: ValueKind<i64>,

    min: ImmOrPNode<i64>,

    max: ImmOrPNode<i64>,

    inc: ImmOrPNode<i64>,

    unit: Option<String>,

    representation: IntegerRepresentation,

    p_selected: Vec<String>,
}

impl IntegerNode {
    pub fn node_base(&self) -> NodeBase<'_> {
        NodeBase::new(&self.attr_base, &self.elem_base)
    }

    pub fn p_invalidators(&self) -> &[String] {
        &self.p_invalidators
    }

    pub fn streamable(&self) -> bool {
        self.streamable
    }

    pub fn value_kind(&self) -> &ValueKind<i64> {
        &self.value_kind
    }

    pub fn min(&self) -> &ImmOrPNode<i64> {
        &self.min
    }

    pub fn max(&self) -> &ImmOrPNode<i64> {
        &self.max
    }

    pub fn inc(&self) -> &ImmOrPNode<i64> {
        &self.inc
    }

    pub fn unit(&self) -> Option<&str> {
        self.unit.as_deref()
    }

    pub fn representation(&self) -> IntegerRepresentation {
        self.representation
    }

    pub fn p_selected(&self) -> &[String] {
        &self.p_selected
    }
}

impl Parse for IntegerNode {
    fn parse(node: &mut xml::Node) -> Self {
        debug_assert!(node.tag_name() == "Integer");

        let attr_base = node.parse();
        let elem_base = node.parse();

        let mut p_invalidators: Vec<String> = vec![];
        while let Some(invalidator) = node.parse_if("pInvalidator") {
            p_invalidators.push(invalidator);
        }

        let streamable = node.parse_if("Streamable").unwrap_or_default();

        let value_kind = node.parse();

        let min = if node.is_next_node_name("Min") || node.is_next_node_name("pMin") {
            Some(node.parse())
        } else {
            None
        };

        let max = if node.is_next_node_name("Max") || node.is_next_node_name("pMax") {
            Some(node.parse())
        } else {
            None
        };

        let inc = if node.is_next_node_name("Inc") || node.is_next_node_name("pInc") {
            node.parse()
        } else {
            ImmOrPNode::Imm(1)
        };

        let unit = node.parse_if("Unit");

        let representation = node
            .parse_if("Representation")
            .unwrap_or_else(|| IntegerRepresentation::PureNumber);

        // Deduce min and max value based on representation if not specified.
        let min = min.unwrap_or_else(|| ImmOrPNode::Imm(representation.deduce_min()));
        let max = max.unwrap_or_else(|| ImmOrPNode::Imm(representation.deduce_max()));

        let mut p_selected: Vec<String> = vec![];
        while let Some(selected) = node.parse_if("pSelected") {
            p_selected.push(selected)
        }

        Self {
            attr_base,
            elem_base,
            p_invalidators,
            streamable,
            value_kind,
            min,
            max,
            inc,
            unit,
            representation,
            p_selected,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn integer_node_from_str(xml: &str) -> IntegerNode {
        let document = xml::Document::from_str(xml).unwrap();
        document.root_node().parse()
    }

    #[test]
    fn test_integer_node_with_immediate() {
        let xml = r#"
            <Integer Name = "TestNode">
                <pInvalidator>Invalidator0</pInvalidator>
                <pInvalidator>Invalidator1</pInvalidator>
                <Streamable>Yes</Streamable>
                <Value>100</Value>
                <Min>10</Min>
                <Max>100</Max>
                <Inc>5</Inc>
                <Unit>dB</Unit>
                <Representation>Logarithmic</Representation>
                <pSelected>Selected0</pSelected>
                <pSelected>Selected1</pSelected>

            </Integer>
            "#;

        let node = integer_node_from_str(xml);

        let p_invalidators = node.p_invalidators();
        assert_eq!(p_invalidators.len(), 2);
        assert_eq!(p_invalidators[0], "Invalidator0");
        assert_eq!(p_invalidators[1], "Invalidator1");

        assert!(node.streamable());
        assert!(matches! {node.value_kind(), ValueKind::Value(100)});
        assert_eq!(node.min(), &ImmOrPNode::Imm(10));
        assert_eq!(node.max(), &ImmOrPNode::Imm(100));
        assert_eq!(node.inc(), &ImmOrPNode::Imm(5));
        assert_eq!(node.unit(), Some("dB"));
        assert_eq!(node.representation(), IntegerRepresentation::Logarithmic);

        let p_selected = node.p_selected();
        assert_eq!(p_selected.len(), 2);
        assert_eq!(p_selected[0], "Selected0");
        assert_eq!(p_selected[1], "Selected1");
    }

    #[test]
    fn test_integer_node_with_p_value() {
        let xml = r#"
            <Integer Name = "TestNode">
                <pValueCopy>Copy1</pValueCopy>
                <pValue>pValue</pValue>
                <pValueCopy>Copy2</pValueCopy>
                <pValueCopy>Copy3</pValueCopy>
                <pMin>pMinNode</pMin>
                <pMax>pMaxNode</pMax>
                <pInc>pIncNode</pInc>
            </Integer>
            "#;

        let node = integer_node_from_str(xml);
        let p_value = match node.value_kind() {
            ValueKind::PValue(p_value) => p_value,
            _ => panic!(),
        };
        assert_eq!(p_value.p_value.as_str(), "pValue");
        let p_value_copies = &p_value.p_value_copies;
        assert_eq!(p_value_copies.len(), 3);
        assert_eq!(p_value_copies[0], "Copy1");
        assert_eq!(p_value_copies[1], "Copy2");
        assert_eq!(p_value_copies[2], "Copy3");

        assert_eq!(node.min(), &ImmOrPNode::PNode("pMinNode".into()));
        assert_eq!(node.max(), &ImmOrPNode::PNode("pMaxNode".into()));
        assert_eq!(node.inc(), &ImmOrPNode::PNode("pIncNode".into()));
    }

    #[test]
    fn test_integer_node_with_p_index() {
        let xml = r#"
        <Integer Name="TestNode">
            <pIndex>pIndexNode</pIndex>
            <ValueIndexed Index="10">100</ValueIndexed>
            <pValueIndexed Index="20">pValueIndexNode</pValueIndexed>
            <ValueIndexed Index="30">300</ValueIndexed>
            <pValueDefault>pValueDefaultNode</pValueDefault>
        </Integer>
        "#;

        let node = integer_node_from_str(xml);
        let p_index = match node.value_kind {
            ValueKind::PIndex(p_index) => p_index,
            _ => panic!(),
        };

        assert_eq!(&p_index.p_index, "pIndexNode");

        let value_indexed = p_index.value_indexed;
        assert_eq!(value_indexed.len(), 3);
        assert!(matches! {value_indexed[0].indexed, ImmOrPNode::Imm(100)});
        assert_eq!(value_indexed[0].index, 10);

        assert_eq!(
            value_indexed[1].indexed,
            ImmOrPNode::PNode("pValueIndexNode".into())
        );
        assert_eq!(value_indexed[1].index, 20);

        assert!(matches! {value_indexed[2].indexed, ImmOrPNode::Imm(300)});
        assert_eq!(value_indexed[2].index, 30);

        assert_eq!(
            p_index.value_default,
            ImmOrPNode::PNode("pValueDefaultNode".into())
        );
    }
}
