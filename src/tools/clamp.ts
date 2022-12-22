export function clamp<Ordered>(val: Ordered, low: Ordered, up: Ordered) {
    if (low > val) {
        return low;
    }
    if (up < val) {
        return up;
    }
    return val;
}