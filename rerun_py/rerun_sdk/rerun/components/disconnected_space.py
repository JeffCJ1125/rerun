# DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/python/mod.rs
# Based on "crates/store/re_types/definitions/rerun/components/disconnected_space.fbs".

# You can extend this class by creating a "DisconnectedSpaceExt" class in "disconnected_space_ext.py".

from __future__ import annotations

from typing_extensions import deprecated  # type: ignore[misc, unused-ignore]

from .. import datatypes
from .._baseclasses import (
    ComponentBatchMixin,
    ComponentDescriptor,
    ComponentMixin,
)
from .disconnected_space_ext import DisconnectedSpaceExt

__all__ = ["DisconnectedSpace", "DisconnectedSpaceBatch"]


@deprecated("""Use [archetypes.Transform3D] with an invalid transform instead.""")
class DisconnectedSpace(DisconnectedSpaceExt, datatypes.Bool, ComponentMixin):
    """
    **Component**: Spatially disconnect this entity from its parent.

    Specifies that the entity path at which this is logged is spatially disconnected from its parent,
    making it impossible to transform the entity path into its parent's space and vice versa.
    It *only* applies to views that work with spatial transformations, i.e. 2D & 3D views.
    This is useful for specifying that a subgraph is independent of the rest of the scene.
    """

    _BATCH_TYPE = None
    # __init__ can be found in disconnected_space_ext.py

    # Note: there are no fields here because DisconnectedSpace delegates to datatypes.Bool
    pass


class DisconnectedSpaceBatch(datatypes.BoolBatch, ComponentBatchMixin):
    _COMPONENT_DESCRIPTOR: ComponentDescriptor = ComponentDescriptor("rerun.components.DisconnectedSpace")


# This is patched in late to avoid circular dependencies.
DisconnectedSpace._BATCH_TYPE = DisconnectedSpaceBatch  # type: ignore[assignment]
