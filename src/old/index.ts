import { Way } from './way'
import { Relation } from './relation'
import { Node } from './node'

export { Way, WayType, TravelMode } from './way'
export { Node } from './node'
export { Relation, RestrictionType, Role, RelationMember } from './relation'
export { Tag, TagMap, Access } from './tag'
export { OsmElement } from './base'
export * from './overpass'

export const enum ItemType {
   Node = 'node',
   Way = 'way',
   Relation = 'relation'
}

/** Decimal latitude and longitude. */
export type Point = [number, number]

/** Left, bottom, right, top */
export type BoundingBox = [number, number, number, number]

/**
 * Box-bounded OSM data download including
 *
 * - All nodes that are inside a given bounding box and any relations that
 *   reference them.
 * - All ways that reference at least one node that is inside a given bounding
 *   box, any relations that reference them [the ways], and any nodes outside
 *   the bounding box that the ways may reference.
 * - All relations that reference one of the nodes, ways or relations included
 *   due to the above rules. (Does not apply recursively.)
 *
 * @see https://wiki.openstreetmap.org/wiki/API_v0.6#Retrieving_map_data_by_bounding_box:_GET_.2Fapi.2F0.6.2Fmap
 */
export interface AreaData {
   /** Nodes keyed to their ID */
   nodes: Map<number, Node>
   /** Ways keyed to their ID */
   ways: Map<number, Way>
   relations: Relation[]
}
