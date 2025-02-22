// DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/rust/api.rs
// Based on "crates/store/re_types/definitions/rerun/blueprint/archetypes/container_blueprint.fbs".

#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(clippy::clone_on_copy)]
#![allow(clippy::cloned_instead_of_copied)]
#![allow(clippy::map_flatten)]
#![allow(clippy::needless_question_mark)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::too_many_lines)]

use ::re_types_core::external::arrow2;
use ::re_types_core::SerializationResult;
use ::re_types_core::{ComponentBatch, ComponentBatchCowWithDescriptor};
use ::re_types_core::{ComponentDescriptor, ComponentName};
use ::re_types_core::{DeserializationError, DeserializationResult};

/// **Archetype**: The description of a container.
#[derive(Clone, Debug)]
pub struct ContainerBlueprint {
    /// The class of the view.
    pub container_kind: crate::blueprint::components::ContainerKind,

    /// The name of the container.
    pub display_name: Option<crate::components::Name>,

    /// `ContainerId`s or `ViewId`s that are children of this container.
    pub contents: Option<Vec<crate::blueprint::components::IncludedContent>>,

    /// The layout shares of each column in the container.
    ///
    /// For [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    pub col_shares: Option<Vec<crate::blueprint::components::ColumnShare>>,

    /// The layout shares of each row of the container.
    ///
    /// For [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers.
    pub row_shares: Option<Vec<crate::blueprint::components::RowShare>>,

    /// Which tab is active.
    ///
    /// Only applies to `Tabs` containers.
    pub active_tab: Option<crate::blueprint::components::ActiveTab>,

    /// Whether this container is visible.
    ///
    /// Defaults to true if not specified.
    pub visible: Option<crate::blueprint::components::Visible>,

    /// How many columns this grid should have.
    ///
    /// If unset, the grid layout will be auto.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal]/[`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    pub grid_columns: Option<crate::blueprint::components::GridColumns>,
}

static REQUIRED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.ContainerKind".into(),
            archetype_field_name: Some("container_kind".into()),
        }]
    });

static RECOMMENDED_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 1usize]> =
    once_cell::sync::Lazy::new(|| {
        [ComponentDescriptor {
            archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
            component_name: "rerun.blueprint.components.ContainerBlueprintIndicator".into(),
            archetype_field_name: None,
        }]
    });

static OPTIONAL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 7usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.components.Name".into(),
                archetype_field_name: Some("display_name".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.IncludedContent".into(),
                archetype_field_name: Some("contents".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.ColumnShare".into(),
                archetype_field_name: Some("col_shares".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.RowShare".into(),
                archetype_field_name: Some("row_shares".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.ActiveTab".into(),
                archetype_field_name: Some("active_tab".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.Visible".into(),
                archetype_field_name: Some("visible".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.GridColumns".into(),
                archetype_field_name: Some("grid_columns".into()),
            },
        ]
    });

static ALL_COMPONENTS: once_cell::sync::Lazy<[ComponentDescriptor; 9usize]> =
    once_cell::sync::Lazy::new(|| {
        [
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.ContainerKind".into(),
                archetype_field_name: Some("container_kind".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.ContainerBlueprintIndicator".into(),
                archetype_field_name: None,
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.components.Name".into(),
                archetype_field_name: Some("display_name".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.IncludedContent".into(),
                archetype_field_name: Some("contents".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.ColumnShare".into(),
                archetype_field_name: Some("col_shares".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.RowShare".into(),
                archetype_field_name: Some("row_shares".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.ActiveTab".into(),
                archetype_field_name: Some("active_tab".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.Visible".into(),
                archetype_field_name: Some("visible".into()),
            },
            ComponentDescriptor {
                archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                component_name: "rerun.blueprint.components.GridColumns".into(),
                archetype_field_name: Some("grid_columns".into()),
            },
        ]
    });

impl ContainerBlueprint {
    /// The total number of components in the archetype: 1 required, 1 recommended, 7 optional
    pub const NUM_COMPONENTS: usize = 9usize;
}

/// Indicator component for the [`ContainerBlueprint`] [`::re_types_core::Archetype`]
pub type ContainerBlueprintIndicator =
    ::re_types_core::GenericIndicatorComponent<ContainerBlueprint>;

impl ::re_types_core::Archetype for ContainerBlueprint {
    type Indicator = ContainerBlueprintIndicator;

    #[inline]
    fn name() -> ::re_types_core::ArchetypeName {
        "rerun.blueprint.archetypes.ContainerBlueprint".into()
    }

    #[inline]
    fn display_name() -> &'static str {
        "Container blueprint"
    }

    #[inline]
    fn indicator() -> ComponentBatchCowWithDescriptor<'static> {
        static INDICATOR: ContainerBlueprintIndicator = ContainerBlueprintIndicator::DEFAULT;
        ComponentBatchCowWithDescriptor::new(&INDICATOR as &dyn ::re_types_core::ComponentBatch)
    }

    #[inline]
    fn required_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        REQUIRED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn recommended_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        RECOMMENDED_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn optional_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        OPTIONAL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn all_components() -> ::std::borrow::Cow<'static, [ComponentDescriptor]> {
        ALL_COMPONENTS.as_slice().into()
    }

    #[inline]
    fn from_arrow2_components(
        arrow_data: impl IntoIterator<Item = (ComponentName, Box<dyn arrow2::array::Array>)>,
    ) -> DeserializationResult<Self> {
        re_tracing::profile_function!();
        use ::re_types_core::{Loggable as _, ResultExt as _};
        let arrays_by_name: ::std::collections::HashMap<_, _> = arrow_data
            .into_iter()
            .map(|(name, array)| (name.full_name(), array))
            .collect();
        let container_kind = {
            let array = arrays_by_name
                .get("rerun.blueprint.components.ContainerKind")
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#container_kind")?;
            <crate::blueprint::components::ContainerKind>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#container_kind")?
                .into_iter()
                .next()
                .flatten()
                .ok_or_else(DeserializationError::missing_data)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#container_kind")?
        };
        let display_name = if let Some(array) = arrays_by_name.get("rerun.components.Name") {
            <crate::components::Name>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#display_name")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let contents =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.IncludedContent") {
                Some({
                    <crate::blueprint::components::IncludedContent>::from_arrow2_opt(&**array)
                        .with_context("rerun.blueprint.archetypes.ContainerBlueprint#contents")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.blueprint.archetypes.ContainerBlueprint#contents")?
                })
            } else {
                None
            };
        let col_shares =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ColumnShare") {
                Some({
                    <crate::blueprint::components::ColumnShare>::from_arrow2_opt(&**array)
                        .with_context("rerun.blueprint.archetypes.ContainerBlueprint#col_shares")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.blueprint.archetypes.ContainerBlueprint#col_shares")?
                })
            } else {
                None
            };
        let row_shares =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.RowShare") {
                Some({
                    <crate::blueprint::components::RowShare>::from_arrow2_opt(&**array)
                        .with_context("rerun.blueprint.archetypes.ContainerBlueprint#row_shares")?
                        .into_iter()
                        .map(|v| v.ok_or_else(DeserializationError::missing_data))
                        .collect::<DeserializationResult<Vec<_>>>()
                        .with_context("rerun.blueprint.archetypes.ContainerBlueprint#row_shares")?
                })
            } else {
                None
            };
        let active_tab =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.ActiveTab") {
                <crate::blueprint::components::ActiveTab>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#active_tab")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        let visible = if let Some(array) = arrays_by_name.get("rerun.blueprint.components.Visible")
        {
            <crate::blueprint::components::Visible>::from_arrow2_opt(&**array)
                .with_context("rerun.blueprint.archetypes.ContainerBlueprint#visible")?
                .into_iter()
                .next()
                .flatten()
        } else {
            None
        };
        let grid_columns =
            if let Some(array) = arrays_by_name.get("rerun.blueprint.components.GridColumns") {
                <crate::blueprint::components::GridColumns>::from_arrow2_opt(&**array)
                    .with_context("rerun.blueprint.archetypes.ContainerBlueprint#grid_columns")?
                    .into_iter()
                    .next()
                    .flatten()
            } else {
                None
            };
        Ok(Self {
            container_kind,
            display_name,
            contents,
            col_shares,
            row_shares,
            active_tab,
            visible,
            grid_columns,
        })
    }
}

impl ::re_types_core::AsComponents for ContainerBlueprint {
    fn as_component_batches(&self) -> Vec<ComponentBatchCowWithDescriptor<'_>> {
        re_tracing::profile_function!();
        use ::re_types_core::Archetype as _;
        [
            Some(Self::indicator()),
            (Some(&self.container_kind as &dyn ComponentBatch)).map(|batch| {
                ::re_types_core::ComponentBatchCowWithDescriptor {
                    batch: batch.into(),
                    descriptor_override: Some(ComponentDescriptor {
                        archetype_name: Some(
                            "rerun.blueprint.archetypes.ContainerBlueprint".into(),
                        ),
                        archetype_field_name: Some(("container_kind").into()),
                        component_name: ("rerun.blueprint.components.ContainerKind").into(),
                    }),
                }
            }),
            (self
                .display_name
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("display_name").into()),
                    component_name: ("rerun.components.Name").into(),
                }),
            }),
            (self
                .contents
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("contents").into()),
                    component_name: ("rerun.blueprint.components.IncludedContent").into(),
                }),
            }),
            (self
                .col_shares
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("col_shares").into()),
                    component_name: ("rerun.blueprint.components.ColumnShare").into(),
                }),
            }),
            (self
                .row_shares
                .as_ref()
                .map(|comp_batch| (comp_batch as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("row_shares").into()),
                    component_name: ("rerun.blueprint.components.RowShare").into(),
                }),
            }),
            (self
                .active_tab
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("active_tab").into()),
                    component_name: ("rerun.blueprint.components.ActiveTab").into(),
                }),
            }),
            (self
                .visible
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("visible").into()),
                    component_name: ("rerun.blueprint.components.Visible").into(),
                }),
            }),
            (self
                .grid_columns
                .as_ref()
                .map(|comp| (comp as &dyn ComponentBatch)))
            .map(|batch| ::re_types_core::ComponentBatchCowWithDescriptor {
                batch: batch.into(),
                descriptor_override: Some(ComponentDescriptor {
                    archetype_name: Some("rerun.blueprint.archetypes.ContainerBlueprint".into()),
                    archetype_field_name: Some(("grid_columns").into()),
                    component_name: ("rerun.blueprint.components.GridColumns").into(),
                }),
            }),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl ::re_types_core::ArchetypeReflectionMarker for ContainerBlueprint {}

impl ContainerBlueprint {
    /// Create a new `ContainerBlueprint`.
    #[inline]
    pub fn new(container_kind: impl Into<crate::blueprint::components::ContainerKind>) -> Self {
        Self {
            container_kind: container_kind.into(),
            display_name: None,
            contents: None,
            col_shares: None,
            row_shares: None,
            active_tab: None,
            visible: None,
            grid_columns: None,
        }
    }

    /// The name of the container.
    #[inline]
    pub fn with_display_name(mut self, display_name: impl Into<crate::components::Name>) -> Self {
        self.display_name = Some(display_name.into());
        self
    }

    /// `ContainerId`s or `ViewId`s that are children of this container.
    #[inline]
    pub fn with_contents(
        mut self,
        contents: impl IntoIterator<Item = impl Into<crate::blueprint::components::IncludedContent>>,
    ) -> Self {
        self.contents = Some(contents.into_iter().map(Into::into).collect());
        self
    }

    /// The layout shares of each column in the container.
    ///
    /// For [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    #[inline]
    pub fn with_col_shares(
        mut self,
        col_shares: impl IntoIterator<Item = impl Into<crate::blueprint::components::ColumnShare>>,
    ) -> Self {
        self.col_shares = Some(col_shares.into_iter().map(Into::into).collect());
        self
    }

    /// The layout shares of each row of the container.
    ///
    /// For [`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers, the length of this list should always match the number of contents.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal] containers.
    #[inline]
    pub fn with_row_shares(
        mut self,
        row_shares: impl IntoIterator<Item = impl Into<crate::blueprint::components::RowShare>>,
    ) -> Self {
        self.row_shares = Some(row_shares.into_iter().map(Into::into).collect());
        self
    }

    /// Which tab is active.
    ///
    /// Only applies to `Tabs` containers.
    #[inline]
    pub fn with_active_tab(
        mut self,
        active_tab: impl Into<crate::blueprint::components::ActiveTab>,
    ) -> Self {
        self.active_tab = Some(active_tab.into());
        self
    }

    /// Whether this container is visible.
    ///
    /// Defaults to true if not specified.
    #[inline]
    pub fn with_visible(
        mut self,
        visible: impl Into<crate::blueprint::components::Visible>,
    ) -> Self {
        self.visible = Some(visible.into());
        self
    }

    /// How many columns this grid should have.
    ///
    /// If unset, the grid layout will be auto.
    ///
    /// Ignored for [`components::ContainerKind::Horizontal`][crate::blueprint::components::ContainerKind::Horizontal]/[`components::ContainerKind::Vertical`][crate::blueprint::components::ContainerKind::Vertical] containers.
    #[inline]
    pub fn with_grid_columns(
        mut self,
        grid_columns: impl Into<crate::blueprint::components::GridColumns>,
    ) -> Self {
        self.grid_columns = Some(grid_columns.into());
        self
    }
}

impl ::re_byte_size::SizeBytes for ContainerBlueprint {
    #[inline]
    fn heap_size_bytes(&self) -> u64 {
        self.container_kind.heap_size_bytes()
            + self.display_name.heap_size_bytes()
            + self.contents.heap_size_bytes()
            + self.col_shares.heap_size_bytes()
            + self.row_shares.heap_size_bytes()
            + self.active_tab.heap_size_bytes()
            + self.visible.heap_size_bytes()
            + self.grid_columns.heap_size_bytes()
    }

    #[inline]
    fn is_pod() -> bool {
        <crate::blueprint::components::ContainerKind>::is_pod()
            && <Option<crate::components::Name>>::is_pod()
            && <Option<Vec<crate::blueprint::components::IncludedContent>>>::is_pod()
            && <Option<Vec<crate::blueprint::components::ColumnShare>>>::is_pod()
            && <Option<Vec<crate::blueprint::components::RowShare>>>::is_pod()
            && <Option<crate::blueprint::components::ActiveTab>>::is_pod()
            && <Option<crate::blueprint::components::Visible>>::is_pod()
            && <Option<crate::blueprint::components::GridColumns>>::is_pod()
    }
}
