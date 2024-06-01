
DROP TABLE IF EXISTS `Breeds`;

CREATE TABLE `Breeds` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `name` varchar(255) NOT NULL,
  `feed_id` bigint unsigned NOT NULL,
  `info` text NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `feed_id` (`feed_id`),
  CONSTRAINT `fk_breeds_feed_id` FOREIGN KEY (`feed_id`) REFERENCES `Feeds` (`id`) ON DELETE CASCADE
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

--
-- Dumping data for table `Breeds`
--

LOCK TABLES `Breeds` WRITE;
/*!40000 ALTER TABLE `Breeds` DISABLE KEYS */;
INSERT INTO `Breeds` VALUES (1,'Merino',1,'Merino sheep are known for their fine wool.'),(2,'Dorper',2,'Dorper sheep are a hardy meat breed.'),(3,'Suffolk',3,'Suffolk sheep are large and primarily raised for meat.');
/*!40000 ALTER TABLE `Breeds` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `FeedSupplies`
--

DROP TABLE IF EXISTS `FeedSupplies`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `FeedSupplies` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `storekeeper_id` bigint unsigned DEFAULT NULL,
  `amount` bigint unsigned NOT NULL,
  `timestamp` bigint unsigned NOT NULL,
  `feed_id` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_feed_supplies_storekeeper_id` (`storekeeper_id`),
  KEY `fk_feed_supplies_feed_id` (`feed_id`),
  CONSTRAINT `fk_feed_supplies_feed_id` FOREIGN KEY (`feed_id`) REFERENCES `Feeds` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_feed_supplies_storekeeper_id` FOREIGN KEY (`storekeeper_id`) REFERENCES `Storekeepers` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=21 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `FeedSupplies`
--

LOCK TABLES `FeedSupplies` WRITE;
/*!40000 ALTER TABLE `FeedSupplies` DISABLE KEYS */;
INSERT INTO `FeedSupplies` VALUES (1,1,5000,1625097600,1),(2,2,8000,1625097600,2),(3,3,6000,1625097600,3),(4,1,7000,1625097600,1),(5,NULL,9000,1625097600,2),(6,3,7500,1625097600,3),(7,1,8500,1625097600,1),(8,2,9500,1625097600,2),(9,3,8000,1625097600,3),(10,1,9000,1625097600,1),(11,2,10000,1625097600,2),(12,3,8500,1625097600,3),(13,1,9500,1625097600,1),(14,2,10500,1625097600,2),(15,3,9000,1625097600,3),(16,1,10000,1625097600,1),(17,2,11000,1625097600,2),(18,3,9500,1625097600,3),(19,1,10500,1625097600,1),(20,NULL,11500,1625097600,2);
/*!40000 ALTER TABLE `FeedSupplies` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `FeedingLogs`
--

DROP TABLE IF EXISTS `FeedingLogs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `FeedingLogs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `sheep_id` bigint unsigned NOT NULL,
  `shepherd_id` bigint unsigned DEFAULT NULL,
  `timestamp` bigint unsigned NOT NULL,
  `feed_id` bigint unsigned NOT NULL,
  `amount` bigint unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_feeding_logs_sheep_id` (`sheep_id`),
  KEY `fk_feeding_logs_shepherd_id` (`shepherd_id`),
  KEY `fk_feeding_logs_feed_id` (`feed_id`),
  CONSTRAINT `fk_feeding_logs_feed_id` FOREIGN KEY (`feed_id`) REFERENCES `Feeds` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_feeding_logs_sheep_id` FOREIGN KEY (`sheep_id`) REFERENCES `Sheep` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_feeding_logs_shepherd_id` FOREIGN KEY (`shepherd_id`) REFERENCES `Shepherds` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=59 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `FeedingLogs`
--

LOCK TABLES `FeedingLogs` WRITE;
/*!40000 ALTER TABLE `FeedingLogs` DISABLE KEYS */;
INSERT INTO `FeedingLogs` VALUES (41,1,1,1625097600,1,500),(42,2,2,1625097600,2,800),(43,3,3,1625097600,3,600),(44,4,NULL,1625097600,1,700),(45,5,2,1625097600,2,900),(46,6,3,1625097600,3,750),(47,7,1,1625097600,1,850),(48,8,NULL,1625097600,2,950),(49,9,3,1625097600,3,800),(50,10,1,1625097600,1,900),(51,11,2,1625097600,2,1000),(52,12,3,1625097600,3,850),(53,13,NULL,1625097600,1,950),(54,14,2,1625097600,2,1050),(55,15,3,1625097600,3,900),(56,16,1,1625097600,1,1000),(57,17,NULL,1625097600,2,1100),(58,18,3,1625097600,3,950);
/*!40000 ALTER TABLE `FeedingLogs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Feeds`
--

DROP TABLE IF EXISTS `Feeds`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Feeds` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `amount` int unsigned NOT NULL,
  `name` varchar(255) NOT NULL,
  `calories` int unsigned NOT NULL,
  `fat` int unsigned NOT NULL,
  `protein` int unsigned NOT NULL,
  `carbohydrates` int unsigned NOT NULL,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=9 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Feeds`
--

LOCK TABLES `Feeds` WRITE;
/*!40000 ALTER TABLE `Feeds` DISABLE KEYS */;
INSERT INTO `Feeds` VALUES (1,10000,'Alfalfa Hay',500,50,200,300),(2,20000,'Corn Silage',600,70,150,380),(3,15000,'Oat Hay',450,40,180,280),(5,3,'string',3,3,3,3),(6,1,'string',1,1,1,1);
/*!40000 ALTER TABLE `Feeds` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `ShearingLogs`
--

DROP TABLE IF EXISTS `ShearingLogs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `ShearingLogs` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `sheep_id` bigint unsigned NOT NULL,
  `shepherd_id` bigint unsigned DEFAULT NULL,
  `timestamp` bigint unsigned NOT NULL,
  `wool_amount` int unsigned NOT NULL,
  PRIMARY KEY (`id`),
  KEY `fk_shearing_logs_sheep_id` (`sheep_id`),
  KEY `fk_shearing_logs_shepherd_id` (`shepherd_id`),
  CONSTRAINT `fk_shearing_logs_sheep_id` FOREIGN KEY (`sheep_id`) REFERENCES `Sheep` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_shearing_logs_shepherd_id` FOREIGN KEY (`shepherd_id`) REFERENCES `Shepherds` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `ShearingLogs`
--

LOCK TABLES `ShearingLogs` WRITE;
/*!40000 ALTER TABLE `ShearingLogs` DISABLE KEYS */;
INSERT INTO `ShearingLogs` VALUES (1,1,1,1625097600,500),(2,2,2,1625097600,800),(3,3,3,1625097600,600),(4,4,NULL,1625097600,700),(5,5,2,1625097600,900),(6,6,3,1625097600,750),(7,7,1,1625097600,850),(8,8,NULL,1625097600,950),(9,9,3,1625097600,800),(10,10,1,1625097600,900),(11,11,2,1625097600,1000),(12,12,3,1625097600,850),(13,13,NULL,1625097600,950),(14,14,2,1625097600,1050),(15,15,3,1625097600,900),(16,16,1,1625097600,1000),(17,17,NULL,1625097600,1100),(18,18,3,1625097600,950);
/*!40000 ALTER TABLE `ShearingLogs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Sheep`
--

DROP TABLE IF EXISTS `Sheep`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Sheep` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `birth_date` bigint unsigned NOT NULL,
  `breed_id` bigint unsigned NOT NULL,
  `sex` tinyint(1) NOT NULL,
  `temperature_scanner_id` bigint unsigned DEFAULT NULL,
  `shepherd_id` bigint unsigned DEFAULT NULL,
  `weight` bigint unsigned NOT NULL DEFAULT '50',
  PRIMARY KEY (`id`),
  UNIQUE KEY `temperature_scanner_id` (`temperature_scanner_id`),
  KEY `fk_sheep_breed_id` (`breed_id`),
  KEY `fk_sheep_shepherd_id` (`shepherd_id`),
  CONSTRAINT `fk_sheep_breed_id` FOREIGN KEY (`breed_id`) REFERENCES `Breeds` (`id`) ON DELETE CASCADE,
  CONSTRAINT `fk_sheep_shepherd_id` FOREIGN KEY (`shepherd_id`) REFERENCES `Shepherds` (`id`) ON DELETE SET NULL,
  CONSTRAINT `fk_sheep_temperature_scanner_id` FOREIGN KEY (`temperature_scanner_id`) REFERENCES `TemperatureScanners` (`id`) ON DELETE SET NULL
) ENGINE=InnoDB AUTO_INCREMENT=19 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Sheep`
--

LOCK TABLES `Sheep` WRITE;
/*!40000 ALTER TABLE `Sheep` DISABLE KEYS */;
INSERT INTO `Sheep` VALUES (1,1622505600,1,1,1,1,50000),(2,1625097600,2,0,2,2,50000),(3,1619827200,3,1,3,3,50000),(4,1612156800,1,0,NULL,NULL,50000),(5,1609478400,2,1,NULL,1,50000),(6,1606790400,3,0,NULL,2,50000),(7,1601510400,1,1,NULL,3,50000),(8,1598918400,2,0,NULL,1,50000),(9,1596230400,3,1,NULL,2,50000),(10,1593542400,1,0,NULL,3,50000),(11,1590864000,2,1,NULL,1,50000),(12,1588185600,3,0,NULL,2,50000),(13,1585804800,1,1,NULL,3,50000),(14,1583126400,2,0,NULL,1,50000),(15,1580534400,3,1,NULL,2,50000),(16,1577856000,1,0,NULL,3,50000),(17,1575177600,2,1,NULL,1,50000),(18,1572595200,3,0,NULL,2,50000);
/*!40000 ALTER TABLE `Sheep` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Shepherds`
--

DROP TABLE IF EXISTS `Shepherds`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Shepherds` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `surname` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Shepherds`
--

LOCK TABLES `Shepherds` WRITE;
/*!40000 ALTER TABLE `Shepherds` DISABLE KEYS */;
INSERT INTO `Shepherds` VALUES (1,'john_smith','password123','John','Smith'),(2,'emily_jones','password456','Emily','Jones'),(3,'michael_brown','password789','Michael','Brown');
/*!40000 ALTER TABLE `Shepherds` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `Storekeepers`
--

DROP TABLE IF EXISTS `Storekeepers`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `Storekeepers` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `username` varchar(255) NOT NULL,
  `password` varchar(255) NOT NULL,
  `name` varchar(255) NOT NULL,
  `surname` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `username` (`username`)
) ENGINE=InnoDB AUTO_INCREMENT=4 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `Storekeepers`
--

LOCK TABLES `Storekeepers` WRITE;
/*!40000 ALTER TABLE `Storekeepers` DISABLE KEYS */;
INSERT INTO `Storekeepers` VALUES (1,'laura_clark','password321','Laura','Clark'),(2,'daniel_miller','password654','Daniel','Miller'),(3,'sarah_davis','password987','Sarah','Davis');
/*!40000 ALTER TABLE `Storekeepers` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `TemperatureScanners`
--

DROP TABLE IF EXISTS `TemperatureScanners`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `TemperatureScanners` (
  `id` bigint unsigned NOT NULL AUTO_INCREMENT,
  `temperature` smallint unsigned DEFAULT NULL,
  `password` varchar(255) NOT NULL,
  PRIMARY KEY (`id`),
  UNIQUE KEY `id` (`id`)
) ENGINE=InnoDB AUTO_INCREMENT=7 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `TemperatureScanners`
--

LOCK TABLES `TemperatureScanners` WRITE;
/*!40000 ALTER TABLE `TemperatureScanners` DISABLE KEYS */;
INSERT INTO `TemperatureScanners` VALUES (1,36,'scannerpass1'),(2,10,'scannerpass2'),(3,35,'scannerpass3'),(4,0,'scannerpass4'),(5,0,'scannerpass5'),(6,0,'scannerpass6');
/*!40000 ALTER TABLE `TemperatureScanners` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2024-05-31  2:48:35
