import { TagMap } from './tag'

/**
 * Properties common to OSM nodes, ways and relations.
 * @see https://wiki.openstreetmap.org/wiki/API_v0.6/XSD
 * @see https://wiki.openstreetmap.org/wiki/Elements
 */
export interface OsmElement {
   /**
    * Used for identifying the element. Element types have their own ID space,
    * so there could be a node with id=100 and a way with id=100, which are
    * unlikely to be related or geographically near to each other.
    *
    * Positive (>0) values are used for all existing elements (and will remain
    * assigned when they are modified or deleted); negative values (<0) are
    * reserved (their scope limited to the current changeset and never stored
    * in the database) and only used when sending data to the OSM database for
    * identifying new objects to create and reference them in other created or
    * modified objects (the server will replace these temporary identifiers sent
    * by the editing application, by assigning an actual positive identifier for
    * each created object, and will return a mapping from the negative
    * identifiers used to their assigned positive identifiers).
    */
   id: number
   //visible?: boolean;
   /**
    * Time of the last modification
    * @example "2016-12-31T23:59:59.999Z"
    */
   timestamp?: number
   /**
    * All types of data element (nodes, ways and relations), as well as
    * changesets, can have tags. Tags describe the meaning of the particular
    * element to which they are attached.
    *
    * A tag consists of two free format text fields; a `key` and a `value`. Each
    * of these are Unicode strings of up to 255 characters. For example,
    * `highway=residential` defines the way as a road whose main function is to
    * give access to people's homes. An element cannot have 2 tags with the same
    * `key`, the key's must be unique. For example, you cannot have an element
    * tagged both `amenity=restaurant` and `amenity=bar`.
    *
    * There is no fixed dictionary of tags, but there are many conventions
    * documented on this wiki (starting with the Map Features page). Tag usage
    * can be measured with the Taginfo application. If there is more than one
    * way to tag a given feature, it's probably best to use the most common
    * approach.
    *
    * Not all elements have tags. Nodes are often untagged if they are part of
    * ways. Both ways and nodes may be untagged if they are members of a
    * relation.
    *
    * @see https://wiki.openstreetmap.org/wiki/Tags
    */
   tags?: TagMap
}

