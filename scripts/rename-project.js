#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

if (process.argv.length < 3) {
    console.error('Usage: node rename-project.js <new-name>');
    console.error('Example: node rename-project.js myapp');
    process.exit(1);
}

const newName = process.argv[2];
const newNameDb = `${newName}_db`;
const newNameTitle = newName.charAt(0).toUpperCase() + newName.slice(1);

// Files to modify
const files = [
    {
        path: 'README.md',
        replacements: [
            { from: 'example_db', to: newNameDb }
        ]
    },
    {
        path: 'backend/migrations/20240101000000_initial.sql',
        replacements: [
            { from: 'examples', to: newName },
            { from: 'idx_examples_name', to: `idx_${newName}_name` },
            { from: 'update_examples_updated_at', to: `update_${newName}_updated_at` }
        ]
    },
    {
        path: 'backend/src/models.rs',
        replacements: [
            { from: 'Example', to: newNameTitle },
            { from: 'CreateExample', to: `Create${newNameTitle}` },
            { from: 'UpdateExample', to: `Update${newNameTitle}` }
        ]
    },
    {
        path: 'frontend/src/App.tsx',
        replacements: [
            { from: 'Example Application', to: `${newNameTitle} Application` }
        ]
    }
];

// Function to replace content in a file
function replaceInFile(filePath, replacements) {
    try {
        let content = fs.readFileSync(filePath, 'utf8');
        replacements.forEach(({ from, to }) => {
            content = content.replace(new RegExp(from, 'g'), to);
        });
        fs.writeFileSync(filePath, content);
        console.log(`Updated ${filePath}`);
    } catch (error) {
        console.error(`Error processing ${filePath}:`, error.message);
    }
}

// Process each file
files.forEach(({ path: filePath, replacements }) => {
    if (fs.existsSync(filePath)) {
        replaceInFile(filePath, replacements);
    } else {
        console.warn(`File not found: ${filePath}`);
    }
});

console.log('\nProject renamed successfully!');
console.log('Next steps:');
console.log('1. Recreate your database with the new name');
console.log('2. Run the migrations again');
console.log('3. Rebuild and restart your application'); 