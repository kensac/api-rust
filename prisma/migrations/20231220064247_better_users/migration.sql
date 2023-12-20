/*
  Warnings:

  - You are about to alter the column `level` on the `Sponsor` table. The data in that column could be lost. The data in that column will be cast from `VarChar(191)` to `Enum(EnumId(2))`.
  - You are about to drop the column `role` on the `User` table. All the data in the column will be lost.

*/
-- AlterTable
ALTER TABLE `Sponsor` MODIFY `level` ENUM('PLATINUM', 'GOLD', 'SILVER', 'BRONZE', 'NONE') NOT NULL DEFAULT 'NONE';

-- AlterTable
ALTER TABLE `User` DROP COLUMN `role`,
    ADD COLUMN `privilege` ENUM('NONE', 'VOLUNTEER', 'TEAM', 'EXEC', 'TECH', 'FINANCE') NOT NULL DEFAULT 'NONE';
