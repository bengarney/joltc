use std::error::Error;
use std::fs::{self, File};
use std::io::{Write};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    bindgen::Builder::default()
        .header("../include/joltc.h")
        .generate()
        .unwrap()
        .write_to_file("src/joltc.rs")
        .unwrap();

    cc::Build::new()
        .cpp(true)
        // .shared_flag(true)
        // .flag("-std=c++17")
        .flag("/std:c++17")
        .define("JPH_DEBUG_RENDERER", None)
        .define("JPH_DEBUG", None)
        .define("JPH_ENABLE_ASSERTS", None)
        .files([
            "../src/joltc.c",
            "../src/joltc_assert.cpp",
            "../src/joltc.cpp",
            "../build/_deps/joltphysics-src/Jolt/AABBTree/AABBTreeBuilder.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/Color.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/Factory.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/IssueReporting.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/JobSystemSingleThreaded.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/JobSystemThreadPool.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/JobSystemWithBarrier.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/LinearCurve.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/Memory.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/Profiler.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/RTTI.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/Semaphore.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/StringTools.cpp",
"../build/_deps/joltphysics-src/Jolt/Core/TickCounter.cpp",
"../build/_deps/joltphysics-src/Jolt/Geometry/ConvexHullBuilder.cpp",
"../build/_deps/joltphysics-src/Jolt/Geometry/ConvexHullBuilder2D.cpp",
"../build/_deps/joltphysics-src/Jolt/Geometry/Indexify.cpp",
"../build/_deps/joltphysics-src/Jolt/Geometry/OrientedBox.cpp",
"../build/_deps/joltphysics-src/Jolt/Math/Vec3.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStream.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStreamBinaryIn.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStreamBinaryOut.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStreamIn.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStreamOut.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStreamTextIn.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/ObjectStreamTextOut.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/SerializableObject.cpp",
"../build/_deps/joltphysics-src/Jolt/ObjectStream/TypeDeclarations.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Body/Body.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Body/BodyCreationSettings.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Body/BodyInterface.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Body/BodyManager.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Body/MassProperties.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Body/MotionProperties.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Character/Character.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Character/CharacterBase.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Character/CharacterVirtual.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/BroadPhase/BroadPhase.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/BroadPhase/BroadPhaseBruteForce.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/BroadPhase/BroadPhaseQuadTree.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/BroadPhase/QuadTree.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/CastConvexVsTriangles.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/CastSphereVsTriangles.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/CollideConvexVsTriangles.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/CollideSphereVsTriangles.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/CollisionDispatch.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/CollisionGroup.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/EstimateCollisionResponse.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/GroupFilter.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/GroupFilterTable.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/ManifoldBetweenTwoFaces.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/NarrowPhaseQuery.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/NarrowPhaseStats.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/PhysicsMaterial.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/PhysicsMaterialSimple.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/BoxShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/CapsuleShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/CompoundShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/ConvexHullShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/ConvexShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/CylinderShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/DecoratedShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/EmptyShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/HeightFieldShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/MeshShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/MutableCompoundShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/OffsetCenterOfMassShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/PlaneShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/RotatedTranslatedShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/ScaledShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/Shape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/SphereShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/StaticCompoundShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/TaperedCapsuleShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/TaperedCylinderShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/Shape/TriangleShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Collision/TransformedShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/ConeConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/Constraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/ConstraintManager.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/ContactConstraintManager.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/DistanceConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/FixedConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/GearConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/HingeConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/MotorSettings.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/PathConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/PathConstraintPath.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/PathConstraintPathHermite.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/PointConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/PulleyConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/RackAndPinionConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/SixDOFConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/SliderConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/SpringSettings.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/SwingTwistConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Constraints/TwoBodyConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/DeterminismLog.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/IslandBuilder.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/LargeIslandSplitter.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/PhysicsScene.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/PhysicsSystem.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/PhysicsUpdateContext.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Ragdoll/Ragdoll.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/SoftBody/SoftBodyCreationSettings.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/SoftBody/SoftBodyMotionProperties.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/SoftBody/SoftBodyShape.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/SoftBody/SoftBodySharedSettings.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/StateRecorderImpl.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/MotorcycleController.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/TrackedVehicleController.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleAntiRollBar.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleCollisionTester.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleConstraint.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleController.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleDifferential.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleEngine.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleTrack.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/VehicleTransmission.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/Wheel.cpp",
"../build/_deps/joltphysics-src/Jolt/Physics/Vehicle/WheeledVehicleController.cpp",
"../build/_deps/joltphysics-src/Jolt/RegisterTypes.cpp",
"../build/_deps/joltphysics-src/Jolt/Renderer/DebugRenderer.cpp",
"../build/_deps/joltphysics-src/Jolt/Renderer/DebugRendererPlayback.cpp",
"../build/_deps/joltphysics-src/Jolt/Renderer/DebugRendererRecorder.cpp",
"../build/_deps/joltphysics-src/Jolt/Renderer/DebugRendererSimple.cpp",
"../build/_deps/joltphysics-src/Jolt/Skeleton/SkeletalAnimation.cpp",
"../build/_deps/joltphysics-src/Jolt/Skeleton/Skeleton.cpp",
"../build/_deps/joltphysics-src/Jolt/Skeleton/SkeletonMapper.cpp",
"../build/_deps/joltphysics-src/Jolt/Skeleton/SkeletonPose.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleGrouper/TriangleGrouperClosestCentroid.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleGrouper/TriangleGrouperMorton.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleSplitter/TriangleSplitter.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleSplitter/TriangleSplitterBinning.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleSplitter/TriangleSplitterFixedLeafSize.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleSplitter/TriangleSplitterLongestAxis.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleSplitter/TriangleSplitterMean.cpp",
"../build/_deps/joltphysics-src/Jolt/TriangleSplitter/TriangleSplitterMorton.cpp",
        ])
        .include("../include")
        .include("../build/_deps/joltphysics-src/")
        .compile("joltc");

csbindgen::Builder::default()
        .input_bindgen_file("src/joltc.rs")
        .method_filter(|x| x.starts_with("JPH"))
        .rust_method_prefix("jpc_")
        .rust_file_header("use crate::joltc::*;")
        .csharp_class_name("JoltC")
        .csharp_namespace("JoltPhysics")
        .csharp_entry_point_prefix("jpc_")
        .csharp_dll_name("libjoltc")
        .csharp_dll_name_if("UNITY_IOS && !UNITY_EDITOR", "__Internal")
        .csharp_method_prefix("")
        .csharp_class_accessibility("public")
        .csharp_generate_const_filter(|_| true)
        .generate_to_file("src/joltc_ffi.rs", "joltc_bindgen.cs")
        .unwrap();

    // Specify the file path
    let file_path = Path::new("joltc_bindgen.cs");

    // Read the file content
    let content = fs::read_to_string(file_path)?;

    // Perform search and replace
    let modified_content = content.replace(" JPH_", " ");

    // Write the modified content back to the file
    let mut file = File::create(file_path)?;
    file.write_all(modified_content.as_bytes())?;

    Ok(())
}