import { ItemType } from '.'

/**
 * Overapass API response
 */
export interface OverpassResponse {
   version: number
   generator: string
   osm3s: any
   elements: OverpassElement[]
}

export interface OverpassElement {
   type: ItemType
   id: number
   tags?: { [key: string]: string }
}

export interface OverpassNode extends OverpassElement {
   type: ItemType.Node
   lat: number
   lon: number
}

export interface OverpassWay extends OverpassElement {
   type: ItemType.Way
   nodes: number[]
}
