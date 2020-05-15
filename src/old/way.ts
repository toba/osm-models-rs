import { OsmElement } from './base'
import { Node } from './node'

/**
 * Collection of nodes representing a way of travel.
 *
 * A way is an ordered list of nodes which normally also has at least one tag or
 * is included within a `relation`. A way can have between 2 and 2,000 nodes,
 * although it's possible that faulty ways with zero or a single node exist. A
 * way can be open or closed. A closed way is one whose last node on the way is
 * also the first on that way. A closed way may be interpreted either as a
 * closed polyline, or an area, or both.
 *
 * @see https://wiki.openstreetmap.org/wiki/Way
 */
export interface Way extends OsmElement {
   nodes: Node[]
}

/**
 * Kind of road, street, path or rail.
 * @see https://wiki.openstreetmap.org/wiki/Key:highway
 */
export const enum WayType {
   /**
    * For horse riders. Equivalent to `highway=path` + `horse=designated`.
    */
   HorsePath = 'bridleway',
   /**
    * For designated cycleways. Add `foot=*` only if default-access-restrictions
    * do not apply.
    */
   BicyclePath = 'cycleway',
   /**
    * For designated footpaths; i.e., mainly/exclusively for pedestrians. This
    * includes walking tracks and gravel paths. If bicycles are allowed as well,
    * you can indicate this by adding a `bicycle=yes` tag. Should not be used
    * for paths where the primary or intended usage is unknown. Use
    * `highway=pedestrian` for pedestrianised roads in shopping or residential
    * areas and `highway=track` if it is usable by agricultural or similar
    * vehicles.
    */
   FootPath = 'footway',
   LightRail = 'light_rail',
   /**
    * A restricted access major divided highway, normally with 2 or more running
    * lanes plus emergency hard shoulder. Equivalent to the Freeway, Autobahn,
    * etc..
    */
   Freeway = 'motorway',
   NarrowGauge = 'narrow_gauge',
   Path = 'path',
   /** Links between larger towns */
   Primary = 'primary',
   Rail = 'rail',
   /**
    * Roads which serve as an access to housing, without function of connecting
    * settlements. Often lined with housing.
    */
   Residential = 'residential',
   /** Links between towns */
   Secondary = 'secondary',
   /**
    * For access roads to, or within an industrial estate, camp site, business
    * park, car park etc. Can be used in conjunction with `service=*` to
    * indicate the type of usage and with access=* to indicate who can use it
    * and in what circumstances.
    */
   ServiceRoad = 'service',
   /**
    * For flights of steps (stairs) on footways. Use with `step_count=*` to
    * indicate the number of steps
    */
   Stairs = 'steps',
   Subway = 'subway',
   Tertiary = 'tertiary',
   /**
    * Roads for mostly agricultural or forestry uses. To describe the quality of
    * a track, see `tracktype=*`. Note: Although tracks are often rough with
    * unpaved surfaces, this tag is not describing the quality of a road but its
    * use. Consequently, if you want to tag a general use road, use one of the
    * general highway values instead of track.
    */
   TwoTrack = 'track',
   Tram = 'tram',
   /**
    * The most important roads in a country's system that aren't motorways.
    * (Need not necessarily be a divided highway.)
    */
   Trunk = 'trunk',
   /**
    * The least important through roads in a country's system — i.e. minor roads
    * of a lower classification than tertiary, but which serve a purpose other
    * than access to properties. (Often link villages and hamlets.)
    *
    * The word 'unclassified' is a historical artefact of the UK road system and
    * does not mean that the classification is unknown; you can use
    * `highway=road` for that.
    */
   Minor = 'unclassified'
}

/**
 * Modes of travel.
 */
export const enum TravelMode {
   Car = 'car',
   Bus = 'bus',
   Bicycle = 'bicycle',
   Horse = 'horse',
   Walk = 'foot',
   Motorcycle = 'motorcycle',
   Tram = 'tram',
   Train = 'train'
}
