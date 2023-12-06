-- DropIndex
DROP INDEX `Scan_hackathonId_fkey` ON `Scan`;

-- DropIndex
DROP INDEX `Score_hackathonId_fkey` ON `Score`;

-- AlterTable
ALTER TABLE `User` ADD COLUMN `role` ENUM('NONE', 'VOLUNTEER', 'TEAM', 'EXEC', 'TECH', 'FINANCE') NOT NULL DEFAULT 'NONE';
