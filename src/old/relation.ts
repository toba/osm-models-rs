import { TagMap } from './tag'
import { Node } from './node'
import { OsmElement } from './base'

export interface RelationMember {
   nodes: Node[]
   /** @see https://wiki.openstreetmap.org/wiki/Relation#Roles */
   role?: Role
}

/**
 * Restrictions and boundaries defined among a collection of nodes.
 * @see https://wiki.openstreetmap.org/wiki/Relation
 * @see https://wiki.openstreetmap.org/wiki/Relation:restriction
 */
export interface Relation extends OsmElement {
   members: RelationMember[]
   /**
    * Tags applied to relation. XPath OSM parsing only allows relations that
    * have tags.
    */
   tags: TagMap
}

/**
 * @see https://wiki.openstreetmap.org/wiki/Relation:restriction
 */
export const enum RestrictionType {
   NoRightTurn = 'no_right_turn',
   NoLeftTurn = 'no_left_turn',
   NoUturn = 'no_u_turn',
   NoStraight = 'no_straight_on',
   OnlyRightTurn = 'only_right_turn',
   OnlyLeftTurn = 'only_left_turn',
   OnlyStright = 'only_straight_on',
   NoEntry = 'no_entry',
   NoExit = 'no_exit'
}

/**
 * Relation member roles.
 */
export const enum Role {
   From = 'from',
   Via = 'via',
   To = 'to',
   /** Relative polygon position */
   Inner = 'inner',
   /** Relative polygon position */
   Outer = 'outer',
   SubArea = 'subarea',
   /** Direction of travel, e.g. for a bus route */
   Forward = 'forward',
   Backward = 'backward',
   Platform = 'platform'
}
