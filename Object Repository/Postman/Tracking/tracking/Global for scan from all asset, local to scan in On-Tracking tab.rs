<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Global for scan from all asset, local to scan in On-Tracking tab</name>
   <tag></tag>
   <elementGuidId>ec49690c-c218-444e-b72c-1c643663c99f</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODNjODFjYzc5OWJkYTE0NDcyYmM4MyIsImlhdCI6MTcxOTkxMjQ3NiwiZXhwIjoxNzE5OTU1Njc2fQ.rWAd1U5y2MPTc7K6ZbqvC-s1iocg0mE0cd55rUCh-co</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>896bc3d4-7afe-472d-841c-a228dacf4a77</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY2NzM5NWJhMDZjNGJjYThiOGRmOWUxZSIsImNvbXBhbnkiOiI2NjQ1YzBjYjcyM2U4ODk3MmE4ZWQzNzkiLCJjb21wYW55Q29kZSI6IjAwMDA2Iiwic2Vzc2lvbiI6IjY2ODNjODFjYzc5OWJkYTE0NDcyYmM4MyIsImlhdCI6MTcxOTkxMjQ3NiwiZXhwIjoxNzE5OTU1Njc2fQ.rWAd1U5y2MPTc7K6ZbqvC-s1iocg0mE0cd55rUCh-co</value>
      <webElementGuid>eeb5fb7b-5da0-48a2-accd-f856de0efda0</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>PUT</restRequestMethod>
   <restUrl>${trckUrl}${trck}scan?tag=C00006Xkah3J&amp;group=6645d64c66796581a5e16f1e&amp;type=Local</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.TRACKING_BASE_URL</defaultValue>
      <description></description>
      <id>469176be-255f-4ffb-b8f7-a6e0001599e9</id>
      <masked>false</masked>
      <name>trckUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.trackingEndpoint</defaultValue>
      <description></description>
      <id>b3f9e21c-e298-411f-800d-46d74c142e12</id>
      <masked>false</masked>
      <name>trck</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Bearer</defaultValue>
      <description></description>
      <id>0c308185-e814-4bfb-a231-18f4fa512c5c</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
