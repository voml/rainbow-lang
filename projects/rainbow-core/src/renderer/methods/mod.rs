use std::fmt::{Debug, Formatter};

use crate::{
    renderer::{RainbowRenderer, RenderFragment},
    vm::RainbowVM,
    RenderNode,
};

mod traits;

impl<'vm> RainbowRenderer<'vm> {
    pub fn new(vm: &'vm RainbowVM, theme: &'vm str, language: &'vm str) -> Self {
        Self { vm, theme, language, tracing: Default::default(), class_name: None }
    }
    pub fn get_class_name(&self) -> String {
        match &self.class_name {
            None => "rainbow".to_string(),
            Some(s) => s.to_string(),
        }
    }
    pub fn set_class_name<S>(&mut self, class_name: S)
    where
        S: Into<String>,
    {
        self.class_name = Some(class_name.into());
    }
    pub fn trace(&mut self, name: &[String]) {
        self.tracing.insert(name.join("-"));
    }
    pub fn clear_tracing(&mut self) {
        self.tracing.clear();
    }
}

impl RenderFragment {
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
    pub fn insert(&mut self, node: RenderNode) {
        self.inner.push(node);
    }
    /// insert multiline texts
    pub fn insert_lines<S>(&mut self, s: S)
    where
        S: Into<String>,
    {
        for line in s.into().lines() {
            if !line.is_empty() {
                self.inner.push(RenderNode::text(line));
            }
        }
    }
    pub fn insert_text<S>(&mut self, s: S)
    where
        S: Into<String>,
    {
        self.inner.push(RenderNode { name: vec![], attributes: Default::default(), text: s.into() });
    }
}

impl RenderNode {
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn text<S>(s: S) -> Self
    where
        S: Into<String>,
    {
        let text = s.into();
        assert_eq!(text.lines().count(), 1, "{:?}", text);
        Self { name: vec![], attributes: Default::default(), text }
    }
}
