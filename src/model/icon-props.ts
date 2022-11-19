export interface IconProps {
    diam: string, 
    src: string, 
    theta?: number, 
    rad_f?: number, 

    rad_total?: string,
    displayed?: boolean,
    triggerEvent?: () => void,
}