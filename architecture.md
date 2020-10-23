# Architecture

This document describes the high-level architecture of the repository and its components.
Note that this is not the full picture but only shows a part at this point.

```
+--------------------+
|                    |
|  Networking Layer  |
|                    |
+---------+----------+
          |
          |
          |
+---------+----------+        +--------------------+        +--------------------+
|                    |        |                    |        |                    |
|    BE Interface    +--------+      Open MLS      +--------+     Persistence    |
|                    |        |                    |        |        Layer       |
+---------+----------+        +---------+----------+        +----------+---------+
          |                             |                              |
          |                             |                              |
          |                             |                              |
+---------------------------------------+------------------------------+-------------+
|                                                                                    |
|    +--------------------+     +--------------------+      +--------------------+   |
|    |                    |     |                    |      |                    |   |
|    |    User & Group    |     |      Message       |      |    User & Group    |   |
|    |     Management     |     |     Management     |      |     Management     |   |
|    +--------------------+     +--------------------+      +--------------------+   |
|                                                                                    |
+------------------------------------------------------------------------------------+
```
