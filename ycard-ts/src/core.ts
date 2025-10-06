import { YCard, ValidationMode, Diagnostic, PhonesStyle } from './types';

/**
 * WASM loader and wrapper for yCard core functionality
 */
class YCardCore {
  private wasmModule: any = null;
  private memory: WebAssembly.Memory | null = null;
  
  async init(): Promise<void> {
    if (this.wasmModule) return;

    try {
      // Load WASM module - in Node.js environment
      if (typeof require !== 'undefined') {
        const wasmPath = require.resolve('../wasm/ycard_core.wasm');
        const fs = require('fs');
        const wasmBytes = fs.readFileSync(wasmPath);
        const wasmModule = await WebAssembly.instantiate(wasmBytes, {});
        this.wasmModule = wasmModule.instance.exports;
        this.memory = this.wasmModule.memory;
      } else {
        // Browser environment
        const wasmUrl = new URL('../wasm/ycard_core.wasm', import.meta.url);
        const wasmModule = await WebAssembly.instantiateStreaming(fetch(wasmUrl));
        this.wasmModule = wasmModule.instance.exports;
        this.memory = this.wasmModule.memory;
      }
    } catch (error) {
      throw new Error(`Failed to load WASM module: ${error}`);
    }
  }

  private writeStringToMemory(str: string): { ptr: number; len: number } {
    if (!this.memory) throw new Error('WASM not initialized');
    
    const encoder = new TextEncoder();
    const bytes = encoder.encode(str);
    
    // Allocate memory (simplified - real implementation would use proper allocator)
    const ptr = this.wasmModule.malloc(bytes.length);
    const memoryArray = new Uint8Array(this.memory.buffer, ptr, bytes.length);
    memoryArray.set(bytes);
    
    return { ptr, len: bytes.length };
  }

  private readStringFromMemory(ptr: number, len: number): string {
    if (!this.memory) throw new Error('WASM not initialized');
    
    const decoder = new TextDecoder();
    const memoryArray = new Uint8Array(this.memory.buffer, ptr, len);
    return decoder.decode(memoryArray);
  }

  async loadAliasPack(packJson: string): Promise<void> {
    await this.init();
    const { ptr, len } = this.writeStringToMemory(packJson);
    const result = this.wasmModule.yc_load_alias_pack(ptr, len);
    this.wasmModule.free(ptr);
    
    if (result !== 0) {
      const errorPtr = this.wasmModule.yc_last_error();
      if (errorPtr !== 0) {
        // Read error message (simplified)
        throw new Error('Failed to load alias pack');
      }
    }
  }

  async setDefaultLocale(locale: string): Promise<void> {
    await this.init();
    const { ptr, len } = this.writeStringToMemory(locale);
    const result = this.wasmModule.yc_set_default_locale(ptr, len);
    this.wasmModule.free(ptr);
    
    if (result !== 0) {
      throw new Error('Failed to set default locale');
    }
  }

  async parse(input: string, locale?: string): Promise<YCard> {
    await this.init();
    const { ptr, len } = this.writeStringToMemory(input);
    const handle = this.wasmModule.yc_parse(ptr, len);
    this.wasmModule.free(ptr);
    
    if (handle === -1) {
      throw new Error('Parse failed');
    }

    // Normalize
    const localePtr = locale ? this.writeStringToMemory(locale) : { ptr: 0, len: 0 };
    const normalizedHandle = this.wasmModule.yc_normalize(handle, 0, localePtr.ptr, localePtr.len);
    if (localePtr.ptr !== 0) this.wasmModule.free(localePtr.ptr);

    // Format to get JSON representation (simplified)
    const formatPtr = this.wasmModule.yc_format(normalizedHandle, 1); // flag 1 for JSON
    if (formatPtr === -1) {
      this.wasmModule.yc_free(handle);
      throw new Error('Format failed');
    }

    // Read result (simplified - would need proper length handling)
    const jsonStr = '{}'; // This would read the actual JSON from memory
    const result = JSON.parse(jsonStr) as YCard;

    this.wasmModule.yc_free(handle);
    return result;
  }

  async format(ycard: YCard, style: PhonesStyle = PhonesStyle.Canonical): Promise<string> {
    await this.init();
    
    // Convert yCard to JSON and parse with WASM
    const jsonStr = JSON.stringify(ycard);
    const { ptr, len } = this.writeStringToMemory(jsonStr);
    const handle = this.wasmModule.yc_parse(ptr, len);
    this.wasmModule.free(ptr);
    
    if (handle === -1) {
      throw new Error('Parse failed during format');
    }

    // Format
    const formatPtr = this.wasmModule.yc_format(handle, 0); // flag 0 for YAML
    if (formatPtr === -1) {
      this.wasmModule.yc_free(handle);
      throw new Error('Format failed');
    }

    // Read formatted result (simplified)
    const result = ''; // This would read the actual YAML from memory
    
    this.wasmModule.yc_free(handle);
    return result;
  }

  async validate(ycard: YCard, mode: ValidationMode = ValidationMode.Lenient): Promise<Diagnostic[]> {
    await this.init();
    
    // Convert yCard to JSON and parse with WASM
    const jsonStr = JSON.stringify(ycard);
    const { ptr, len } = this.writeStringToMemory(jsonStr);
    const handle = this.wasmModule.yc_parse(ptr, len);
    this.wasmModule.free(ptr);
    
    if (handle === -1) {
      throw new Error('Parse failed during validation');
    }

    // Validate
    const diagnosticsPtr = this.wasmModule.yc_validate(handle, mode);
    if (diagnosticsPtr === -1) {
      this.wasmModule.yc_free(handle);
      throw new Error('Validation failed');
    }

    // Read diagnostics JSON (simplified)
    const diagnosticsJson = '[]'; // This would read the actual JSON from memory
    const result = JSON.parse(diagnosticsJson) as Diagnostic[];
    
    this.wasmModule.yc_free(handle);
    return result;
  }
}

// Singleton instance
const coreInstance = new YCardCore();

// Public API functions
export async function loadAliasPack(packJson: string): Promise<void> {
  return coreInstance.loadAliasPack(packJson);
}

export async function setDefaultLocale(locale: string): Promise<void> {
  return coreInstance.setDefaultLocale(locale);
}

export async function parse(input: string, locale?: string): Promise<YCard> {
  return coreInstance.parse(input, locale);
}

export async function format(ycard: YCard, style: PhonesStyle = PhonesStyle.Canonical): Promise<string> {
  return coreInstance.format(ycard, style);
}

export async function validate(ycard: YCard, mode: ValidationMode = ValidationMode.Lenient): Promise<Diagnostic[]> {
  return coreInstance.validate(ycard, mode);
}