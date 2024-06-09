namespace Time

open Fable.Core
[<Erase; Emit("functor_runtime_common::FrameTime")>] 
type FrameTime =
    
    [<Emit("$0.dts")>]
    abstract dts: float32
    [<Emit("$0.tts")>]
    abstract tts: float32