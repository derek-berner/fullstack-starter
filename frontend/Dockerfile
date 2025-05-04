FROM node:18-alpine

WORKDIR /app

# Install dependencies first (better caching)
COPY package*.json ./
RUN npm install

# Copy the rest of the code
COPY . .

# Install dependencies again to ensure they're in the container
RUN npm install

EXPOSE 3000

# Use nodemon for development
RUN npm install -g nodemon

# Start the development server
CMD ["npm", "start"] 