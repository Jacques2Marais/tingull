let invokeFunction = () => console.log('invoking nothing!');

if (!import.meta.env.SSR) {
    const { invoke } = await import('@tauri-apps/api/tauri');
    invokeFunction = invoke;
}

export async function invoke(name, args) {
    return await invokeFunction(name, args);
}