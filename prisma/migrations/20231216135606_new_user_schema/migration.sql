-- CreateTable
CREATE TABLE `Event` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `type` ENUM('Activity', 'Food', 'Workshop', 'checkIn') NOT NULL,
    `description` VARCHAR(191) NOT NULL,
    `locationId` VARCHAR(191) NOT NULL,
    `icon` VARCHAR(191) NULL,
    `startTime` DATETIME(3) NOT NULL,
    `endTime` DATETIME(3) NOT NULL,
    `wsPresenterNames` VARCHAR(191) NULL,
    `wsRelevantSkills` VARCHAR(191) NULL,
    `wsSkillLevel` VARCHAR(191) NULL,
    `wsUrls` VARCHAR(191) NULL,
    `hackathonId` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `ExtraCreditAssignment` (
    `registraionID` VARCHAR(191) NOT NULL,
    `classId` VARCHAR(191) NOT NULL,

    UNIQUE INDEX `ExtraCreditAssignment_registraionID_classId_key`(`registraionID`, `classId`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `ExtraCreditClass` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `hackathonId` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Hackathon` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `startTime` DATETIME(3) NOT NULL,
    `endTime` DATETIME(3) NOT NULL,
    `active` BOOLEAN NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Location` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Organizer` (
    `id` VARCHAR(191) NOT NULL,
    `firstName` VARCHAR(191) NOT NULL,
    `lastName` VARCHAR(191) NOT NULL,
    `email` VARCHAR(191) NOT NULL,
    `privilege` ENUM('NONE', 'VOLUNTEER', 'TEAM', 'EXEC', 'TECH', 'FINANCE') NOT NULL,
    `gcpId` VARCHAR(191) NOT NULL,

    UNIQUE INDEX `Organizer_email_key`(`email`),
    UNIQUE INDEX `Organizer_gcpId_key`(`gcpId`),
    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Project` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `hackathonId` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Registration` (
    `id` VARCHAR(191) NOT NULL,
    `userId` VARCHAR(191) NOT NULL,
    `gender` VARCHAR(191) NOT NULL,
    `phone` VARCHAR(191) NOT NULL,
    `country` VARCHAR(191) NOT NULL,
    `race` VARCHAR(191) NULL,
    `travelReimbursement` BOOLEAN NOT NULL,
    `driving` BOOLEAN NOT NULL,
    `firstHackathon` BOOLEAN NOT NULL,
    `academicYear` VARCHAR(191) NOT NULL,
    `educationalInstitutionType` VARCHAR(191) NOT NULL,
    `codingExperience` VARCHAR(191) NULL,
    `shirtSize` VARCHAR(191) NOT NULL,
    `dietaryRestrictions` VARCHAR(191) NULL,
    `allergies` VARCHAR(191) NULL,
    `eighteenBeforeEvent` BOOLEAN NOT NULL,
    `mlhCoc` BOOLEAN NOT NULL,
    `mlhDcp` BOOLEAN NOT NULL,
    `reference` VARCHAR(191) NULL,
    `resume` VARCHAR(191) NULL,
    `university` VARCHAR(191) NOT NULL,
    `major` VARCHAR(191) NOT NULL,
    `projectId` VARCHAR(191) NULL,
    `expectations` VARCHAR(191) NULL,
    `shareAddressMlh` BOOLEAN NULL,
    `shareAddressSponsors` BOOLEAN NULL,
    `shareEmailMlh` BOOLEAN NULL,
    `veteran` BOOLEAN NOT NULL,
    `hackathonId` VARCHAR(191) NOT NULL,
    `time` DATETIME(3) NOT NULL,

    UNIQUE INDEX `Registration_userId_hackathonId_key`(`userId`, `hackathonId`),
    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Scan` (
    `eventId` VARCHAR(191) NOT NULL,
    `registrationId` VARCHAR(191) NOT NULL,
    `organizerId` VARCHAR(191) NOT NULL,
    `hackathonId` VARCHAR(191) NOT NULL,

    UNIQUE INDEX `Scan_eventId_registrationId_key`(`eventId`, `registrationId`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Sponsor` (
    `id` VARCHAR(191) NOT NULL,
    `name` VARCHAR(191) NOT NULL,
    `level` VARCHAR(191) NOT NULL,
    `link` VARCHAR(191) NULL,
    `darkLogo` VARCHAR(191) NOT NULL,
    `lightLogo` VARCHAR(191) NOT NULL,
    `order` INTEGER NOT NULL,
    `hackathonId` VARCHAR(191) NOT NULL,

    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `User` (
    `id` VARCHAR(191) NOT NULL,
    `firstName` VARCHAR(191) NOT NULL,
    `lastName` VARCHAR(191) NOT NULL,
    `email` VARCHAR(191) NOT NULL,
    `gcpId` VARCHAR(191) NOT NULL,
    `role` ENUM('NONE', 'VOLUNTEER', 'TEAM', 'EXEC', 'TECH', 'FINANCE') NOT NULL DEFAULT 'NONE',

    UNIQUE INDEX `User_email_key`(`email`),
    UNIQUE INDEX `User_gcpId_key`(`gcpId`),
    PRIMARY KEY (`id`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- CreateTable
CREATE TABLE `Score` (
    `hackathonId` VARCHAR(191) NOT NULL,
    `projectID` VARCHAR(191) NOT NULL,
    `judgeID` VARCHAR(191) NOT NULL,
    `submitted` BOOLEAN NOT NULL,
    `creativity` INTEGER NOT NULL,
    `technicality` INTEGER NOT NULL,
    `implementation` INTEGER NOT NULL,
    `clarity` INTEGER NOT NULL,
    `growth` INTEGER NOT NULL,
    `challenge1` INTEGER NOT NULL,
    `challenge2` INTEGER NOT NULL,
    `challenge3` INTEGER NOT NULL,

    UNIQUE INDEX `Score_projectID_judgeID_key`(`projectID`, `judgeID`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `Event` ADD CONSTRAINT `Event_locationId_fkey` FOREIGN KEY (`locationId`) REFERENCES `Location`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Event` ADD CONSTRAINT `Event_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `ExtraCreditAssignment` ADD CONSTRAINT `ExtraCreditAssignment_registraionID_fkey` FOREIGN KEY (`registraionID`) REFERENCES `Registration`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

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
ALTER TABLE `Scan` ADD CONSTRAINT `Scan_registrationId_fkey` FOREIGN KEY (`registrationId`) REFERENCES `Registration`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Scan` ADD CONSTRAINT `Scan_organizerId_fkey` FOREIGN KEY (`organizerId`) REFERENCES `Organizer`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Scan` ADD CONSTRAINT `Scan_eventId_fkey` FOREIGN KEY (`eventId`) REFERENCES `Event`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Sponsor` ADD CONSTRAINT `Sponsor_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Score` ADD CONSTRAINT `Score_projectID_fkey` FOREIGN KEY (`projectID`) REFERENCES `Project`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Score` ADD CONSTRAINT `Score_judgeID_fkey` FOREIGN KEY (`judgeID`) REFERENCES `Organizer`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;
