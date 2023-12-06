/*
  Warnings:

  - The primary key for the `Event` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `ExtraCreditClass` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Hackathon` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Location` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Project` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `Registration` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `codingExoerience` on the `Registration` table. All the data in the column will be lost.
  - You are about to drop the column `judgeId` on the `Score` table. All the data in the column will be lost.
  - You are about to drop the column `projectId` on the `Score` table. All the data in the column will be lost.
  - You are about to drop the column `technical` on the `Score` table. All the data in the column will be lost.
  - The primary key for the `Sponsor` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - The primary key for the `User` table will be changed. If it partially fails, the table could be left without primary key constraint.
  - You are about to drop the column `allergies` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `dietaryRestrictions` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `major` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `resume` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `shirtSize` on the `User` table. All the data in the column will be lost.
  - You are about to drop the column `university` on the `User` table. All the data in the column will be lost.
  - A unique constraint covering the columns `[email]` on the table `Organizer` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[gcpId]` on the table `Organizer` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[projectID,judgeID]` on the table `Score` will be added. If there are existing duplicate values, this will fail.
  - A unique constraint covering the columns `[gcpId]` on the table `User` will be added. If there are existing duplicate values, this will fail.
  - Added the required column `gcpId` to the `Organizer` table without a default value. This is not possible if the table is not empty.
  - Added the required column `major` to the `Registration` table without a default value. This is not possible if the table is not empty.
  - Added the required column `shirtSize` to the `Registration` table without a default value. This is not possible if the table is not empty.
  - Added the required column `university` to the `Registration` table without a default value. This is not possible if the table is not empty.
  - Added the required column `judgeID` to the `Score` table without a default value. This is not possible if the table is not empty.
  - Added the required column `projectID` to the `Score` table without a default value. This is not possible if the table is not empty.
  - Added the required column `technicality` to the `Score` table without a default value. This is not possible if the table is not empty.
  - Added the required column `gcpId` to the `User` table without a default value. This is not possible if the table is not empty.

*/
-- DropForeignKey
ALTER TABLE `Event` DROP FOREIGN KEY `Event_hackathonId_fkey`;

-- DropForeignKey
ALTER TABLE `Event` DROP FOREIGN KEY `Event_locationId_fkey`;

-- DropForeignKey
ALTER TABLE `ExtraCreditAssignment` DROP FOREIGN KEY `ExtraCreditAssignment_classId_fkey`;

-- DropForeignKey
ALTER TABLE `ExtraCreditAssignment` DROP FOREIGN KEY `ExtraCreditAssignment_userId_fkey`;

-- DropForeignKey
ALTER TABLE `ExtraCreditClass` DROP FOREIGN KEY `ExtraCreditClass_hackathonId_fkey`;

-- DropForeignKey
ALTER TABLE `Project` DROP FOREIGN KEY `Project_hackathonId_fkey`;

-- DropForeignKey
ALTER TABLE `Registration` DROP FOREIGN KEY `Registration_hackathonId_fkey`;

-- DropForeignKey
ALTER TABLE `Registration` DROP FOREIGN KEY `Registration_userId_fkey`;

-- DropForeignKey
ALTER TABLE `Scan` DROP FOREIGN KEY `Scan_hackathonId_fkey`;

-- DropForeignKey
ALTER TABLE `Scan` DROP FOREIGN KEY `Scan_userId_fkey`;

-- DropForeignKey
ALTER TABLE `Score` DROP FOREIGN KEY `Score_hackathonId_fkey`;

-- DropForeignKey
ALTER TABLE `Score` DROP FOREIGN KEY `Score_judgeId_fkey`;

-- DropForeignKey
ALTER TABLE `Sponsor` DROP FOREIGN KEY `Sponsor_hackathonId_fkey`;

-- DropIndex
DROP INDEX `Score_projectId_judgeId_key` ON `Score`;

-- AlterTable
ALTER TABLE `Event` DROP PRIMARY KEY,
    MODIFY `id` VARCHAR(191) NOT NULL,
    MODIFY `locationId` VARCHAR(191) NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `ExtraCreditAssignment` MODIFY `userId` VARCHAR(191) NOT NULL,
    MODIFY `classId` VARCHAR(191) NOT NULL;

-- AlterTable
ALTER TABLE `ExtraCreditClass` DROP PRIMARY KEY,
    MODIFY `id` VARCHAR(191) NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `Hackathon` DROP PRIMARY KEY,
    MODIFY `id` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `Location` DROP PRIMARY KEY,
    MODIFY `id` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `Organizer` ADD COLUMN `gcpId` VARCHAR(191) NOT NULL;

-- AlterTable
ALTER TABLE `Project` DROP PRIMARY KEY,
    MODIFY `id` VARCHAR(191) NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `Registration` DROP PRIMARY KEY,
    DROP COLUMN `codingExoerience`,
    ADD COLUMN `allergies` VARCHAR(191) NULL,
    ADD COLUMN `codingExperience` VARCHAR(191) NULL,
    ADD COLUMN `dietaryRestrictions` VARCHAR(191) NULL,
    ADD COLUMN `major` VARCHAR(191) NOT NULL,
    ADD COLUMN `projectId` VARCHAR(191) NULL,
    ADD COLUMN `resume` VARCHAR(191) NULL,
    ADD COLUMN `shirtSize` VARCHAR(191) NOT NULL,
    ADD COLUMN `university` VARCHAR(191) NOT NULL,
    MODIFY `id` VARCHAR(191) NOT NULL,
    MODIFY `userId` VARCHAR(191) NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `Scan` MODIFY `eventId` VARCHAR(191) NOT NULL,
    MODIFY `userId` VARCHAR(191) NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL;

-- AlterTable
ALTER TABLE `Score` DROP COLUMN `judgeId`,
    DROP COLUMN `projectId`,
    DROP COLUMN `technical`,
    ADD COLUMN `judgeID` VARCHAR(191) NOT NULL,
    ADD COLUMN `projectID` VARCHAR(191) NOT NULL,
    ADD COLUMN `technicality` INTEGER NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL;

-- AlterTable
ALTER TABLE `Sponsor` DROP PRIMARY KEY,
    MODIFY `id` VARCHAR(191) NOT NULL,
    MODIFY `hackathonId` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- AlterTable
ALTER TABLE `User` DROP PRIMARY KEY,
    DROP COLUMN `allergies`,
    DROP COLUMN `dietaryRestrictions`,
    DROP COLUMN `major`,
    DROP COLUMN `resume`,
    DROP COLUMN `shirtSize`,
    DROP COLUMN `university`,
    ADD COLUMN `gcpId` VARCHAR(191) NOT NULL,
    MODIFY `id` VARCHAR(191) NOT NULL,
    ADD PRIMARY KEY (`id`);

-- CreateIndex
CREATE UNIQUE INDEX `Organizer_email_key` ON `Organizer`(`email`);

-- CreateIndex
CREATE UNIQUE INDEX `Organizer_gcpId_key` ON `Organizer`(`gcpId`);

-- CreateIndex
CREATE UNIQUE INDEX `Score_projectID_judgeID_key` ON `Score`(`projectID`, `judgeID`);

-- CreateIndex
CREATE UNIQUE INDEX `User_gcpId_key` ON `User`(`gcpId`);

-- AddForeignKey
ALTER TABLE `Event` ADD CONSTRAINT `Event_locationId_fkey` FOREIGN KEY (`locationId`) REFERENCES `Location`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Event` ADD CONSTRAINT `Event_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `ExtraCreditAssignment` ADD CONSTRAINT `ExtraCreditAssignment_userId_fkey` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `ExtraCreditAssignment` ADD CONSTRAINT `ExtraCreditAssignment_classId_fkey` FOREIGN KEY (`classId`) REFERENCES `ExtraCreditClass`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `ExtraCreditClass` ADD CONSTRAINT `ExtraCreditClass_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Project` ADD CONSTRAINT `Project_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Registration` ADD CONSTRAINT `Registration_userId_fkey` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Registration` ADD CONSTRAINT `Registration_projectId_fkey` FOREIGN KEY (`projectId`) REFERENCES `Project`(`id`) ON DELETE SET NULL ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Registration` ADD CONSTRAINT `Registration_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Scan` ADD CONSTRAINT `Scan_userId_fkey` FOREIGN KEY (`userId`) REFERENCES `User`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Scan` ADD CONSTRAINT `Scan_eventId_fkey` FOREIGN KEY (`eventId`) REFERENCES `Event`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Sponsor` ADD CONSTRAINT `Sponsor_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Score` ADD CONSTRAINT `Score_projectID_fkey` FOREIGN KEY (`projectID`) REFERENCES `Project`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Score` ADD CONSTRAINT `Score_judgeID_fkey` FOREIGN KEY (`judgeID`) REFERENCES `Organizer`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;
