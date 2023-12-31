//import org.jetbrains.kotlin.gradle.tasks.KotlinCompile
import com.github.imflog.schema.registry.tasks.register.RegisterSchemasTask

//
buildscript {
    repositories {
        gradlePluginPortal()
        maven("https://packages.confluent.io/maven/")
        maven("https://jitpack.io")
    }
}

repositories {
    mavenCentral()
    gradlePluginPortal()
    maven("https://packages.confluent.io/maven/")
    maven("https://jitpack.io")
    mavenLocal()
}

plugins {
    // https://plugins.gradle.org/plugin/org.jetbrains.kotlin.jvm
    kotlin("jvm")

    // https://github.com/ImFlog/schema-registry-plugin/blob/master/README.md
    id("com.github.imflog.kafka-schema-registry-gradle-plugin") version "1.11.1"
}

val protosPath = "claims-schema-setter/src/main/resources/proto/"


schemaRegistry {
    url = "http://localhost:58003/"
    quiet = true

    register {
        // Register schema for each entity separately
        subject("claimStatus", protosPath + "claim.proto", "PROTOBUF")
        subject("incidentType", protosPath + "claim.proto", "PROTOBUF")
        subject("claim", protosPath + "claim.proto", "PROTOBUF")
//            .addReference("claimStatus.proto", "claimStatus", -1)
//            .addReference("incidentType.proto", "incidentType", -1)

        // Register schema for test topic "claims.test-value"
        subject("claims.test-value", protosPath + "claim.proto", "PROTOBUF")
//            .addReference("claimStatus.proto", "claimStatus", -1)
//            .addReference("incidentType.proto", "incidentType", -1)

        // Register schema for test topic "claimsdb.claim.events" outbox table topic
        subject("claimsdb.claim.events-value", protosPath + "claim.proto", "PROTOBUF")
//            .addReference("claimStatus.proto", "claimStatus", -1)
//            .addReference("incidentType.proto", "incidentType", -1)

        // Register schema for test topic "claimsdb.claim.events" outbox table topic
        subject("claimsdb.party.events-value", protosPath + "party.proto", "PROTOBUF")
    }
}

group = rootProject.group
version = rootProject.version


java.sourceCompatibility = JavaVersion.VERSION_17

// Override schema registry url with SCHAME_REGISTRY_URL environment variable if given
tasks.withType<RegisterSchemasTask> {
    schemaRegistry {
        url = System.getenv("SCHEMA_REGISTRY_URL") ?: schemaRegistry.url.get()
    }
}