package org.lettras

import com.dylibso.chicory.compiler.MachineFactoryCompiler
import com.dylibso.chicory.runtime.Instance
import com.dylibso.chicory.wasm.Parser
import com.dylibso.chicory.wasm.WasmModule

/**
 * Runs the compiled Lettras engine (WebAssembly, C interface) inside Chicory.
 *
 * On the JVM the module is compiled to bytecode once, which is far faster than interpreting it. Android's
 * runtime cannot load generated JVM bytecode, so there (or if compilation fails) the interpreter is used.
 * Each thread keeps its own instance: setup is paid once per thread, and calls never share state.
 */
internal class Engine(wasm: ByteArray, useCompiler: Boolean = true) {
    private val module: WasmModule = Parser.parse(wasm)

    private val compiled: java.util.function.Function<Instance, com.dylibso.chicory.runtime.Machine>? =
        if (!useCompiler || isAndroid()) null else runCatching { MachineFactoryCompiler.compile(module) }.getOrNull()

    /** True when running on the bytecode compiler, false on the interpreter. */
    val isCompiled: Boolean get() = compiled != null

    private val instances = ThreadLocal<Instance>()

    private fun instance(): Instance = instances.get() ?: newInstance().also { instances.set(it) }

    private fun newInstance(): Instance {
        val builder = Instance.builder(module)
        compiled?.let { builder.withMachineFactory(it) }
        return builder.build()
    }

    /** Returns the puzzle JSON, or throws [LettrasException] with the engine's message. */
    fun generate(requestJson: String): String {
        val instance = instance()
        val memory = instance.memory()
        val alloc = instance.export("lettras_alloc")
        val free = instance.export("lettras_free")
        val generate = instance.export("lettras_generate")
        val resultPtr = instance.export("lettras_result_ptr")

        val input = requestJson.toByteArray(Charsets.UTF_8)
        val ptr = alloc.apply(input.size.toLong())[0].toInt()
        memory.write(ptr, input)
        val r = generate.apply(ptr.toLong(), input.size.toLong())[0]
        free.apply(ptr.toLong(), input.size.toLong())

        val length = kotlin.math.abs(r).toInt()
        val out = memory.readBytes(resultPtr.apply()[0].toInt(), length).toString(Charsets.UTF_8)
        if (r < 0) throw LettrasException(out)
        return out
    }

    private fun isAndroid(): Boolean =
        System.getProperty("java.vendor")?.contains("Android", ignoreCase = true) == true ||
            System.getProperty("java.vm.name")?.contains("Dalvik", ignoreCase = true) == true
}
