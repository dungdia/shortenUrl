CREATE DATABASE  IF NOT EXISTS `shortenurl` /*!40100 DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_ai_ci */ /*!80016 DEFAULT ENCRYPTION='N' */;
USE `shortenurl`;
-- MySQL dump 10.13  Distrib 8.0.41, for Win64 (x86_64)
--
-- Host: localhost    Database: shortenurl
-- ------------------------------------------------------
-- Server version	8.4.4

/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!50503 SET NAMES utf8 */;
/*!40103 SET @OLD_TIME_ZONE=@@TIME_ZONE */;
/*!40103 SET TIME_ZONE='+00:00' */;
/*!40014 SET @OLD_UNIQUE_CHECKS=@@UNIQUE_CHECKS, UNIQUE_CHECKS=0 */;
/*!40014 SET @OLD_FOREIGN_KEY_CHECKS=@@FOREIGN_KEY_CHECKS, FOREIGN_KEY_CHECKS=0 */;
/*!40101 SET @OLD_SQL_MODE=@@SQL_MODE, SQL_MODE='NO_AUTO_VALUE_ON_ZERO' */;
/*!40111 SET @OLD_SQL_NOTES=@@SQL_NOTES, SQL_NOTES=0 */;

--
-- Table structure for table `access_logs`
--

DROP TABLE IF EXISTS `access_logs`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `access_logs` (
  `id` int NOT NULL AUTO_INCREMENT,
  `url_id` int DEFAULT NULL,
  `visitor_ip` varchar(45) DEFAULT NULL,
  `user_agent` text,
  `accessed_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  KEY `url_id` (`url_id`),
  CONSTRAINT `access_logs_ibfk_1` FOREIGN KEY (`url_id`) REFERENCES `urls` (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `access_logs`
--

LOCK TABLES `access_logs` WRITE;
/*!40000 ALTER TABLE `access_logs` DISABLE KEYS */;
/*!40000 ALTER TABLE `access_logs` ENABLE KEYS */;
UNLOCK TABLES;

--
-- Table structure for table `urls`
--

DROP TABLE IF EXISTS `urls`;
/*!40101 SET @saved_cs_client     = @@character_set_client */;
/*!50503 SET character_set_client = utf8mb4 */;
CREATE TABLE `urls` (
  `id` int NOT NULL AUTO_INCREMENT,
  `long_url` text NOT NULL,
  `short_code` varchar(10) NOT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`),
  UNIQUE KEY `idx_short_code` (`short_code`)
) ENGINE=InnoDB AUTO_INCREMENT=6 DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;
/*!40101 SET character_set_client = @saved_cs_client */;

--
-- Dumping data for table `urls`
--

LOCK TABLES `urls` WRITE;
/*!40000 ALTER TABLE `urls` DISABLE KEYS */;
INSERT INTO `urls` VALUES (2,'https://drive.google.com/drive/folders/1mBSvn4HJDgu4oQkoS_FDf2fEzvGW0-SZ','4ouGLY-I1F','2026-03-14 03:53:19'),(3,'https://svsguedu-my.sharepoint.com/:w:/g/personal/3122410054_sv_sgu_edu_vn/IQDkHG9-m7_dSYHn88oHEFqVAT3vYRLzA_NlRoG2wCeb3uc?e=K6Ze7N','Py2elz3tHh','2026-03-14 06:30:35'),(4,'https://www.youtube.com/watch?v=CDW2ey5z7r4','_AE849jsju','2026-03-17 14:46:29'),(5,'https://youtu.be/dQw4w9WgXcQ?si=yk785J_qZ0U9wVTP','zfR0F9Nfv2','2026-03-17 14:48:21');
/*!40000 ALTER TABLE `urls` ENABLE KEYS */;
UNLOCK TABLES;
/*!40103 SET TIME_ZONE=@OLD_TIME_ZONE */;

/*!40101 SET SQL_MODE=@OLD_SQL_MODE */;
/*!40014 SET FOREIGN_KEY_CHECKS=@OLD_FOREIGN_KEY_CHECKS */;
/*!40014 SET UNIQUE_CHECKS=@OLD_UNIQUE_CHECKS */;
/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
/*!40111 SET SQL_NOTES=@OLD_SQL_NOTES */;

-- Dump completed on 2026-03-17 22:56:42
