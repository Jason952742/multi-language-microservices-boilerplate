# Repository

Code related to storage warehousing services. The warehousing schema typically includes a warehousing interface and a warehousing implementation service. They work together to complete the aggregation of DO domain objects within the persistence , or based on the aggregation of the root ID query , to complete the aggregation of entities and value objects and other DO domain objects such as data initialisation .
In addition, the storage catalogue will have persistence objects PO, and persistence implementation logic related code, such as DAO. There is an important principle in the storage design , that is, an aggregation can only have a storage.

In principle, warehousing should belong to the basic layer, but in order to ensure the convenience of reorganisation of the aggregation code in the evolution of microservices architecture, here the warehousing related code is also put into the domain layer of the aggregation directory.
