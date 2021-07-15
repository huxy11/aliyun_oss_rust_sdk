use headers_serializer::ToMaps;

#[derive(Clone, Debug, Default, PartialEq, ToMaps)]
// #[cfg_attr(feature = "deserialize_structs", derive(Deserialize))]
pub struct GetBucketOptions {
    ///<p>The character used to group objects by name. If you specify the delimiter parameter in the request, the response contains the CommonPrefixes element. Objects whose names contain the same string that ranges from the prefix to the next occurrence of the delimiter are grouped as a single result element in CommonPrefixes.</p>
    #[label("opts")]
    pub delimiter: Option<String>,
    ///<p>The name of the object from which the list operation begins. If this parameter is specified, objects whose names are alphabetically greater than the marker parameter value are returned.</p>
    ///<p>The marker parameter is used to list the returned objects by page, and its value must be smaller than 1,024 bytes in length.</p>
    ///<p>Even if the specified marker does not exist in the list during a conditional query, the list starts from the object whose name is alphabetically greater than the marker parameter value.</p>
    #[label("opts")]
    pub marker: Option<String>,
    ///<p>The maximum number of objects that can be returned for a list operation. If the objects cannot be completely listed at one time because max-keys is specified, a NextMarker element is included in the response as the marker for the next list operation.</p>
    ///<p>Valid values: 1 to 1000</p>
    #[label("opts")]
    pub max_keys: Option<String>,
    ///<p>The prefix that the returned object names must contain.</p>
    ///<p><li>The value of prefix must be smaller than 1,024 bytes in length.</li>
    ///<li>If you specify a prefix to query objects, the names of the returned objects still contain the prefix.</li></p>
    ///<p>If prefix is set to a folder name in the request, the objects whose names contain this prefix are listed, including all objects and subfolders in the folder.</p>
    ///<p>If prefix is specified and delimiter is set to a forward slash (/), only the objects in the folder are listed. The subfolders are grouped together as a single result in CommonPrefixes. Objects and folders in the subfolders are not listed.</p>
    ///<p>For example, a bucket contains the following objects: fun/test.jpg, fun/movie/001.avi, and fun/movie/007.avi. If prefix is set to fun/, the three objects are all returned. If prefix is set to fun/ and delimiter is set to a forward slash (/), fun/test.jpg and fun/movie/are returned.</p>
    #[label("opts")]
    pub prefix: Option<String>,
    ///<p>The encoding type of the object name in the response.</p>
    #[label("opts")]
    pub encoding_type: Option<String>,
}
