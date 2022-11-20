export function get_quant_unit(size: string): [number, string] {
    const size_arr = size.split("")
    const digs = size_arr.filter((c) => {
        return (c <= "9") && (c >= "0")
    })
    const unit_arr = size_arr.slice(digs.length)
    const unit = unit_arr.join("")
    const quant = Number(digs?.join(""))
    return [quant, unit]
}