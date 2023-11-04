-- CreateTable
CREATE TABLE `Score` (
    `hackathonId` INTEGER NOT NULL,
    `projectId` INTEGER NOT NULL,
    `judgeId` VARCHAR(191) NOT NULL,
    `submitted` BOOLEAN NOT NULL,
    `creativity` INTEGER NOT NULL,
    `technical` INTEGER NOT NULL,
    `implementation` INTEGER NOT NULL,
    `clarity` INTEGER NOT NULL,
    `growth` INTEGER NOT NULL,
    `challenge1` INTEGER NOT NULL,
    `challenge2` INTEGER NOT NULL,
    `challenge3` INTEGER NOT NULL,

    UNIQUE INDEX `Score_projectId_judgeId_key`(`projectId`, `judgeId`)
) DEFAULT CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci;

-- AddForeignKey
ALTER TABLE `Score` ADD CONSTRAINT `Score_hackathonId_fkey` FOREIGN KEY (`hackathonId`) REFERENCES `Hackathon`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE `Score` ADD CONSTRAINT `Score_judgeId_fkey` FOREIGN KEY (`judgeId`) REFERENCES `Organizer`(`id`) ON DELETE RESTRICT ON UPDATE CASCADE;
