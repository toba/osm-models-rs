/**
 * Common tags relevant to routing.
 */
export const enum Tag {
   /** @see https://wiki.openstreetmap.org/wiki/Key:access */
   Access = 'access',
   /** `WayType` value */
   RoadType = 'highway',
   RailType = 'railway',
   JunctionType = 'junction',
   OneWay = 'oneway',
   /** Restriction exceptions */
   Exception = 'except',
   Type = 'type',
   /**
    * Indicates access generally disallowed or disallowed for a specific mode
    * of transportation.
    * @see https://wiki.openstreetmap.org/wiki/Relation:restriction
    */
   Restriction = 'restriction',
   Bicycle = 'bicycle',
   Bus = 'bus',
   Foot = 'foot',
   Horse = 'horse',
   MotorCar = 'motorcar',
   Motorcycle = 'motorcycle',
   MotorVehicle = 'motor_vehicle',
   ServiceVehicle = 'psv',
   Vehicle = 'vehicle'
}

/**
 * @see https://wiki.openstreetmap.org/wiki/Key:access
 */
export const enum Access {
   /** Access only for agricultural vehicles */
   Agricultural = 'agricultural',
   /** Public has an official, legally-enshrined right of access */
   Allowed = 'yes',
   /** Accewss only for customers */
   Customers = 'customers',
   /** Access only for deliveries */
   Delivery = 'delivery',
   /** Access is legal but discouraged */
   Discouraged = 'discouraged',
   /** Access only to specific destination */
   Destination = 'destination',
   /** Only forestry traffic allowed */
   Forestry = 'forestry',
   /** No access to general public */
   None = 'no',
   /** Owner granted access */
   Permissive = 'permissive',
   /** Accessible only to individuals with permission */
   Private = 'private'
}

/** Values keyed to tags. */
export type TagMap = { [key: string]: string | undefined }
