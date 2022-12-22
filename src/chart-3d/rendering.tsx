import { Md5 } from "ts-md5";

export function rerender_updated<Prop>(
    props: Prop[], 
    old_hashes: React.MutableRefObject<(string | Int32Array)[]>, 
    entity_gener: (prop_w_index: [Prop, number]) => JSX.Element,
    entities: JSX.Element[] )
{
    const new_hashes = gen_new_hashes(props)
    const updated = updated_props(new_hashes, old_hashes, props)
    exhaust_entity_info(updated, entity_gener, entities)
    old_hashes.current = new_hashes
}
function hash_equals(a: (string | Int32Array), b: (string | Int32Array)): boolean {
    if (a.length != b.length) {
        return false;
    }
    for (let i = 0; i < a.length; i += 1) {
        const a_val = a[i] as string
        const b_val = b[i] as string
        if (a_val != b_val) {
            return false;
        }
    }
    return true;
}
function exhaust_entity_info<Prop>(
    queue: [Prop, number][], 
    entity_gener: (info: [Prop, number]) => JSX.Element, 
    render_dump: JSX.Element[])
{
    while (queue.length > 0) {
        const info_index = queue.pop() as [Prop, number]
        const [_, index] = info_index
        const entity = entity_gener(info_index)

        if (index < render_dump.length) {
            render_dump[index] = entity
        } else {
            render_dump.fill(<group></group>, render_dump.length, index)
            render_dump.push(entity)
        }
    }
}
function gen_new_hashes<Prop>(props: Prop[]) {
    return props.map((prop) => {
        const hash = new Md5()
        hash.start()
        hash.appendStr(JSON.stringify(prop))
        return hash.end() || ""
    }) || []
}
function updated_props<Prop>(
    new_hashes: (string | Int32Array)[], 
    old_hashes: React.MutableRefObject<(string | Int32Array)[]>, 
    props: Prop[],
    )
{
    const updated_indices = new_hashes.map((hash, i) => [hash, i] as [(string | Int32Array), number])
        .filter(([new_hash, i]) => {
            const old_hash = old_hashes.current[i] || ""
            return !hash_equals(new_hash, old_hash)
        })
    return updated_indices.map(([_, i]) => [props[i], i] as [Prop, number])
}