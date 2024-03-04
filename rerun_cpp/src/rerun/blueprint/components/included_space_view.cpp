// DO NOT EDIT! This file was auto-generated by crates/re_types_builder/src/codegen/cpp/mod.rs
// Based on "crates/re_types/definitions/rerun/blueprint/components/included_space_view.fbs".

#include "included_space_view.hpp"

#include "../../datatypes/uuid.hpp"

#include <arrow/builder.h>
#include <arrow/type_fwd.h>

namespace rerun::blueprint::components {}

namespace rerun {
    const std::shared_ptr<arrow::DataType>&
        Loggable<blueprint::components::IncludedSpaceView>::arrow_datatype() {
        static const auto datatype = Loggable<rerun::datatypes::Uuid>::arrow_datatype();
        return datatype;
    }

    rerun::Error Loggable<blueprint::components::IncludedSpaceView>::fill_arrow_array_builder(
        arrow::FixedSizeListBuilder* builder,
        const blueprint::components::IncludedSpaceView* elements, size_t num_elements
    ) {
        static_assert(
            sizeof(rerun::datatypes::Uuid) == sizeof(blueprint::components::IncludedSpaceView)
        );
        RR_RETURN_NOT_OK(Loggable<rerun::datatypes::Uuid>::fill_arrow_array_builder(
            builder,
            reinterpret_cast<const rerun::datatypes::Uuid*>(elements),
            num_elements
        ));

        return Error::ok();
    }

    Result<std::shared_ptr<arrow::Array>>
        Loggable<blueprint::components::IncludedSpaceView>::to_arrow(
            const blueprint::components::IncludedSpaceView* instances, size_t num_instances
        ) {
        // TODO(andreas): Allow configuring the memory pool.
        arrow::MemoryPool* pool = arrow::default_memory_pool();
        auto datatype = arrow_datatype();

        ARROW_ASSIGN_OR_RAISE(auto builder, arrow::MakeBuilder(datatype, pool))
        if (instances && num_instances > 0) {
            RR_RETURN_NOT_OK(
                Loggable<blueprint::components::IncludedSpaceView>::fill_arrow_array_builder(
                    static_cast<arrow::FixedSizeListBuilder*>(builder.get()),
                    instances,
                    num_instances
                )
            );
        }
        std::shared_ptr<arrow::Array> array;
        ARROW_RETURN_NOT_OK(builder->Finish(&array));
        return array;
    }
} // namespace rerun
