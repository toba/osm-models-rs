import { OsmElement } from './base'

/**
 * A node is one of the core elements in the OpenStreetMap data model. It
 * consists of a single point in space defined by its latitude, longitude and
 * node id.
 *
 * A third, optional dimension (altitude) can also be included: `key:ele`
 * (abrev. for "elevation"). A node can also be defined as part of a particular
 * `layer=*` or `level=*`, where distinct features pass over or under one
 * another; say, at a bridge.
 *
 * @see https://wiki.openstreetmap.org/wiki/Node
 */
export interface Node extends OsmElement {
   /**
    * Latitude coordinate in degrees (North of equator is positive) using the
    * standard WGS84 projection
    */
   lat: number
   /**
    * Longitude coordinate in degrees (East of Greenwich is positive) using the
    * standard WGS84 projection. Note that the geographic poles will be exactly
    * at latitude ±90 degrees but in that case the longitude will be set to an
    * arbitrary value within this range.
    */
   lon: number
   /** Altitude or elevation */
   ele?: number
   open?: boolean
   date?: number
   point(): [number, number]
}
