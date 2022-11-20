export interface IconProps {
    diam: string,
    src: string,
    theta?: number,
    rad_f?: number,
    activColor?: string,
    sec_trans?: number,

    rad_total?: string,
    displayed?: boolean,
    triggerEvent?: () => void,
}