# near-lake

**NB! The project is in early stage of development and shouldn't be used in production environmental yet**

NEAR Lake is an indexer built on top of [NEAR Indexer microframework](https://github.com/nearprotocol/nearcore/tree/master/chain/indexer)
to watch the network and store all the events as JSON files on AWS S3.

## Concept

We used to have [NEAR Indexer for Explorer](https://github.com/near/near-indexer-for-explorer) that was watching for 
the network and stored all the events to PostgreSQL database. PostgreSQL became the main bottleneck for us. After some
brainstorming sessions and researches we decided to go with [SingleStore](https://www.singlestore.com/) database. 

Knowing the fact that [NEAR Explorer](https://explorer.near.org) is not the only project that uses the Indexer for Explorer's 
database, we wanted to come up with the concept that will allow us to cover even more projects that can benefit from the data 
from NEAR Protocol.

That's why we decided to store the data from the blockchain as JSON files on AWS S3 bucket that can be used
as a data source for different projects. 

As "Indexer for Explorer Remake" project we are going to have `near-lake` as a data writer. There's going to be
another project that will read from AWS S3 bucket and will store all the data in SingleStore database. This
will replace NEAR Indexer for Explorer PostgreSQL database at some moment and will become the main
source for NEAR Explorer.

